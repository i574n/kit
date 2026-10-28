param(
    [string] $BaseUrl = 'http://localhost:3000',
    [string] $Username = 'iggy',
    [ValidateRange(2, 10000000)] [int] $Count = 10000,
    [ValidateRange(1, 100000)] [int] $Batch = 1000,
    [string] $RestartCommand,
    [ValidateRange(10, 3600)] [int] $ReadySeconds = 300
)
$ErrorActionPreference = 'Stop'
if (-not $env:IGGY_PASSWORD) { throw 'verify.ps1: set IGGY_PASSWORD (the root password from secret iggy/iggy-root)' }
$stream = "verify-$([DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())"
$partitionZero = [Convert]::ToBase64String([BitConverter]::GetBytes([uint32] 0))

function Wait-Ping {
    $deadline = (Get-Date).AddSeconds($ReadySeconds)
    while ($true) {
        try { if ((Invoke-WebRequest "$BaseUrl/ping" -TimeoutSec 5 -UseBasicParsing).StatusCode -eq 200) { return } } catch { }
        if ((Get-Date) -gt $deadline) { throw "verify.ps1: $BaseUrl/ping not answering after $ReadySeconds s" }
        Start-Sleep -Seconds 2
    }
}

function Get-Headers {
    $login = Invoke-RestMethod "$BaseUrl/users/login" -Method Post -ContentType 'application/json' -Body (@{ username = $Username; password = $env:IGGY_PASSWORD } | ConvertTo-Json)
    @{ Authorization = "Bearer $($login.access_token.token)" }
}

function Invoke-Iggy([string] $method, [string] $path, $body) {
    $request = @{ Uri = "$BaseUrl/$path"; Method = $method; Headers = $script:headers; ContentType = 'application/json' }
    if ($null -ne $body) { $request.Body = $body | ConvertTo-Json -Depth 8 -Compress }
    Invoke-RestMethod @request
}

function Receive-Next([int] $want) {
    $polled = Invoke-Iggy Get "streams/$stream/topics/t/messages?consumer_id=1&partition_id=0&kind=next&value=0&count=$want&auto_commit=true" $null
    @($polled.messages | ForEach-Object { [int] ([Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($_.payload))) })
}

Wait-Ping
$script:headers = Get-Headers
Invoke-Iggy Post 'streams' @{ name = $stream } | Out-Null
Invoke-Iggy Post "streams/$stream/topics" @{ name = 't'; partitions_count = 1; compression_algorithm = 'none'; max_topic_size = 0; message_expiry = 0 } | Out-Null

$sendWatch = [Diagnostics.Stopwatch]::StartNew()
for ($first = 0; $first -lt $Count; $first += $Batch) {
    $last = [Math]::Min($Count, $first + $Batch) - 1
    $messages = foreach ($index in $first..$last) { @{ id = 0; payload = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes([string] $index)) } }
    Invoke-Iggy Post "streams/$stream/topics/t/messages" @{ partitioning = @{ kind = 'partition_id'; value = $partitionZero }; messages = @($messages) } | Out-Null
}
$sendWatch.Stop()

$received = [System.Collections.Generic.List[int]]::new()
$pollWatch = [Diagnostics.Stopwatch]::StartNew()
$half = [int] ($Count / 2)
while ($received.Count -lt $half) {
    $chunk = Receive-Next ([Math]::Min($Batch, $half - $received.Count))
    if (-not $chunk.Count) { throw "verify.ps1: poll returned nothing at $($received.Count) of $half" }
    $received.AddRange([int[]] $chunk)
}

$restarted = 'no restart'
if ($RestartCommand) {
    & ([scriptblock]::Create($RestartCommand))
    if ($LASTEXITCODE) { throw "verify.ps1: restart command failed (exit $LASTEXITCODE)" }
    Start-Sleep -Seconds 5
    Wait-Ping
    $script:headers = Get-Headers
    $restarted = 'restarted mid-stream'
}

$empty = 0
while ($received.Count -lt $Count) {
    $chunk = Receive-Next $Batch
    if (-not $chunk.Count) { $empty++; if ($empty -gt 3) { break }; Start-Sleep -Milliseconds 500; continue }
    $empty = 0
    $received.AddRange([int[]] $chunk)
}
$pollWatch.Stop()

$seen = [System.Collections.Generic.HashSet[int]]::new()
$firstDeliveries = [System.Collections.Generic.List[int]]::new()
foreach ($value in $received) { if ($seen.Add($value)) { $firstDeliveries.Add($value) } }
$duplicates = $received.Count - $firstDeliveries.Count
$missing = $Count - $firstDeliveries.Count
$ordered = $true
for ($index = 1; $index -lt $firstDeliveries.Count; $index++) { if ($firstDeliveries[$index] -le $firstDeliveries[$index - 1]) { $ordered = $false; break } }
Invoke-Iggy Delete "streams/$stream" $null | Out-Null

$summary = "sent $Count in {0:N1} s ({1:N0} msg/s), received $($received.Count) ($duplicates duplicates, $missing missing, ordered $ordered) in {2:N1} s, $restarted" -f $sendWatch.Elapsed.TotalSeconds, ($Count / [Math]::Max(0.001, $sendWatch.Elapsed.TotalSeconds)), $pollWatch.Elapsed.TotalSeconds
if ($missing -eq 0 -and $ordered) { "VERIFY-OK $summary"; exit 0 }
"VERIFY-FAILED $summary"
exit 1
