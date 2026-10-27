param(
    [string] $Distro = 'Ubuntu',
    [ValidateRange(2, 256)] [int] $MemoryGB = 6,
    [string] $K3sVersion = 'v1.37.1+k3s1',
    [string] $IggyImage = 'apache/iggy:0.9.0',
    [string] $RootUsername = 'iggy',
    [ValidateRange(10, 3600)] [int] $ReadySeconds = 300
)
$ErrorActionPreference = 'Stop'
$env:WSL_UTF8 = '1'

function Invoke-Distro([string] $command) {
    wsl.exe -d $Distro -u root -- sh -c $command
    if ($LASTEXITCODE) { throw "node-wsl.ps1: '$command' failed in $Distro (exit $LASTEXITCODE)" }
}

function Set-IniValue([System.Collections.Generic.List[string]] $lines, [string] $section, [string] $key, [string] $value) {
    $header = [Array]::FindIndex($lines.ToArray(), [Predicate[string]] { param($line) $line.Trim() -eq "[$section]" })
    if ($header -lt 0) {
        $lines.Add("[$section]")
        $lines.Add("$key=$value")
        return $true
    }
    $end = $header + 1
    while ($end -lt $lines.Count -and -not $lines[$end].Trim().StartsWith('[')) { $end++ }
    for ($index = $header + 1; $index -lt $end; $index++) {
        if ($lines[$index] -match "^\s*$([regex]::Escape($key))\s*=") {
            if ($lines[$index].Trim() -eq "$key=$value") { return $false }
            $lines[$index] = "$key=$value"
            return $true
        }
    }
    $lines.Insert($end, "$key=$value")
    $true
}

if (@(wsl.exe -l -q | ForEach-Object { $_.Trim() }) -notcontains $Distro) { throw "node-wsl.ps1: no WSL distro '$Distro' (wsl.exe -l -q)" }

$wslConfig = Join-Path $HOME '.wslconfig'
$lines = [System.Collections.Generic.List[string]]::new()
if (Test-Path $wslConfig) { foreach ($line in [IO.File]::ReadAllLines($wslConfig)) { $lines.Add($line) } }
$changed = $false
foreach ($setting in @(@('wsl2', 'memory', "${MemoryGB}GB"), @('wsl2', 'swap', '0'), @('wsl2', 'vmIdleTimeout', '-1'), @('general', 'instanceIdleTimeout', '-1'))) {
    if (Set-IniValue $lines $setting[0] $setting[1] $setting[2]) { $changed = $true }
}
if ($changed) {
    [IO.File]::WriteAllText($wslConfig, ($lines -join "`n") + "`n")
    "wslconfig: memory=${MemoryGB}GB swap=0, idle timeouts off; restarting WSL"
    wsl.exe --shutdown
}

$recipe = "$(wsl.exe -d $Distro -u root -- wslpath -a $PSScriptRoot.Replace('\', '/'))".Trim()
if (-not $recipe.StartsWith('/')) { throw "node-wsl.ps1: wslpath could not map $PSScriptRoot into $Distro" }
Invoke-Distro "K3S_VERSION='$K3sVersion' sh '$recipe/bootstrap.sh'"
Invoke-Distro "k3s kubectl create namespace iggy --dry-run=client -o yaml | k3s kubectl apply -f - >/dev/null"
Invoke-Distro "k3s kubectl -n iggy get secret iggy-root >/dev/null 2>&1 || k3s kubectl -n iggy create secret generic iggy-root --from-literal=username='$RootUsername' --from-literal=password=`"`$(head -c 24 /dev/urandom | od -An -tx1 | tr -d ' \n')`" >/dev/null"
Invoke-Distro "k3s kubectl apply -f '$recipe/iggy.yaml'"
Invoke-Distro "k3s kubectl -n iggy set image deployment/iggy iggy='$IggyImage' >/dev/null"
Invoke-Distro "k3s kubectl -n iggy rollout status deployment/iggy --timeout=${ReadySeconds}s"

$deadline = (Get-Date).AddSeconds($ReadySeconds)
while ($true) {
    try {
        $ping = Invoke-WebRequest 'http://localhost:3000/ping' -TimeoutSec 5 -UseBasicParsing
        if ($ping.StatusCode -eq 200) { break }
    } catch { }
    if ((Get-Date) -gt $deadline) { throw "node-wsl.ps1: http://localhost:3000/ping not answering after $ReadySeconds s" }
    Start-Sleep -Seconds 3
}
$k3s = (wsl.exe -d $Distro -u root -- sh -c 'k3s --version | head -n 1').Trim()
"IGGY-READY $IggyImage on $k3s in WSL '$Distro' (memory cap ${MemoryGB}GB): http://localhost:3000, tcp localhost:8090, quic udp 8080, websocket 8092; root credentials in secret iggy/iggy-root"
