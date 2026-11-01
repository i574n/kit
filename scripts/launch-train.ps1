param(
    [Parameter(Mandatory)][string] $Name,
    [switch] $Snapshot,
    [switch] $SkipRegen,
    [switch] $Resume,
    [string] $RunnerHash = 'F1DD2E34F1E816C8',
    [int] $CellsPort = 13977,
    [int] $HoldWaitMinutes = 240
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'user-env.ps1')
$kitRoot = Split-Path $PSScriptRoot -Parent
$kit = Join-Path $env:LOCALAPPDATA 'kit\bin\kit.exe'
$runner = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\spiral\workspace\target\release\spiral.exe')).Path
$hash = (Get-FileHash -LiteralPath $runner).Hash.Substring(0, 16)
if ($RunnerHash -and $hash -ne $RunnerHash) { throw "Cells runner is $hash; this train pins $RunnerHash (pass -RunnerHash '' to accept any)" }
$battery = Get-CimInstance Win32_Battery | Select-Object -First 1
if ($battery -and $battery.BatteryStatus -ne 2) { throw 'the box is on battery; trains run on AC power' }
$python = [Environment]::GetEnvironmentVariable('SPIRAL_PYTHON', 'User')
if ($python) { $env:Path = "$(Split-Path $python);$env:Path" }
$env:SPIRAL_CELLS_SHELL_LAUNCHER = $kit
$env:SPIRAL_CELLS_COMPILER_PORT = "$CellsPort"
if (Get-NetTCPConnection -LocalPort $CellsPort -State Listen -ErrorAction SilentlyContinue) { throw "port $CellsPort is taken; pass another -CellsPort" }
$holdDir = Join-Path $env:LOCALAPPDATA 'kit\train-holds'
$holdDeadline = (Get-Date).AddMinutes($HoldWaitMinutes)
while ($true) {
    $live = @(Get-ChildItem -LiteralPath $holdDir -Filter *.hold -File -ErrorAction SilentlyContinue | ForEach-Object {
        $fields = @{}
        foreach ($line in Get-Content -LiteralPath $_.FullName) { $at = $line.IndexOf('='); if ($at -gt 0) { $fields[$line.Substring(0, $at).Trim()] = $line.Substring($at + 1).Trim() } }
        $until = [datetime]::MinValue
        $holderPid = 0
        $alive = [int]::TryParse("$($fields['pid'])", [ref] $holderPid) -and [bool](Get-Process -Id $holderPid -ErrorAction SilentlyContinue)
        $current = [datetime]::TryParse("$($fields['until'])", [ref] $until) -and (Get-Date) -lt $until
        if ($alive -or $current) { "$($fields['holder']) until $($fields['until'])" }
    })
    if (-not $live.Count) { break }
    if ((Get-Date) -ge $holdDeadline) { throw "trains are held ($($live -join '; ')); waited $HoldWaitMinutes min" }
    [Console]::WriteLine("$(Get-Date -Format HH:mm) train held: $($live -join '; '); waiting")
    Start-Sleep 60
}
$trainArgs = @('-NoProfile', '-File', (Join-Path $PSScriptRoot 'release-train.ps1'), '-Name', $Name)
if ($Snapshot) { $trainArgs += '-Snapshot' }
if ($SkipRegen) { $trainArgs += '-SkipRegen' }
if ($Resume) { $trainArgs += '-Resume' }
if ($Snapshot -or $Resume) {
    $libCopy = Join-Path $kitRoot ".cache/release-tree/$Name-lib/lib/spiral"
    & robocopy (Join-Path $kitRoot '..\spiral\lib\spiral') $libCopy /MIR /XD bench target /NFL /NDL /NJH /NJS /NP | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "lib copy failed (robocopy exit $LASTEXITCODE)" }
    $trainArgs += @('-Lib', $libCopy)
}
Set-Location $kitRoot
& $kit job start --heavy "kit-$Name" 18000000 pwsh @trainArgs