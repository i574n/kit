param([string] $Label = '', [int] $Minutes = 1, [int] $LocalPort = 18090, [string] $Distro = 'Ubuntu', [string] $Tool = 'follow', [string[]] $ToolArguments = @())
$ErrorActionPreference = 'Stop'
$toolPath = Join-Path $env:LOCALAPPDATA "kit\iggy-bin\$Tool.exe"
if ($Tool -eq 'follow') { if (-not $Label) { 'STOP: follow needs -Label'; exit 1 }; $ToolArguments = @($Label, "$Minutes") }
$address = (Get-Content (Join-Path $HOME '.ssh\laptop-last-ip.txt') -TotalCount 1).Trim()
$encoded = (ssh -o "HostName=$address" laptop "wsl.exe -d $Distro -u root -- sh -c `"k3s kubectl -n iggy get secret iggy-root -o jsonpath='{.data.password}'`"" | Out-String).Trim()
if (-not $encoded) { 'STOP: could not read secret iggy/iggy-root over ssh'; exit 1 }
$env:IGGY_PASSWORD = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($encoded))
$env:IGGY_ADDRESS = "127.0.0.1:$LocalPort"
$tunnel = Start-Process -FilePath ssh -ArgumentList '-N', '-o', "HostName=$address", '-o', 'ExitOnForwardFailure=yes', '-L', "${LocalPort}:127.0.0.1:8090", 'laptop' -PassThru -NoNewWindow
try {
    $open = $false
    foreach ($attempt in 1..20) {
        if ($tunnel.HasExited) { break }
        $probe = [Net.Sockets.TcpClient]::new()
        try { $probe.Connect('127.0.0.1', $LocalPort); $open = $true } catch { } finally { $probe.Dispose() }
        if ($open) { break }
        Start-Sleep -Milliseconds 250
    }
    if (-not $open) { "STOP: tunnel to the node's iggy did not open on 127.0.0.1:$LocalPort (ssh exited: $($tunnel.HasExited))"; exit 1 }
    $clock = [Diagnostics.Stopwatch]::StartNew()
    $lines = @(& $toolPath @ToolArguments 2>&1 | ForEach-Object { "$_" })
    $code = $LASTEXITCODE
    $clock.Stop()
    $lines
    "review-box $Tool $($ToolArguments -join ' '): exit $code after $($clock.ElapsedMilliseconds) ms, step events $(@($lines | Where-Object { $_ -match '^step ' }).Count)"
    exit $code
}
finally {
    if (-not $tunnel.HasExited) { Stop-Process -Id $tunnel.Id -Force }
}
