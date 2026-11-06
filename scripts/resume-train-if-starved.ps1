param(
    [Parameter(Mandatory)][string] $Name,
    [string] $RunnerHash = '2E5F532E73EEAF14'
)
$ErrorActionPreference = 'Stop'
$kitRoot = Split-Path $PSScriptRoot -Parent
$out = Join-Path $kitRoot ".cache/release-train/$Name"
$log = Join-Path $out 'train.log'
$stop = @(Select-String -LiteralPath $log -Pattern '^\S+ STOP: (.*)$' | ForEach-Object { $_.Matches[0].Groups[1].Value }) | Select-Object -Last 1
if (-not $stop) {
    if (Select-String -LiteralPath $log -Pattern '^\S+ DONE' -Quiet) { [Console]::WriteLine("$Name finished; nothing to resume"); exit 0 }
    [Console]::WriteLine("$Name ended without STOP or DONE (stopped mid-phase, e.g. by its job's wall-clock cap); resuming from its logs")
    & (Join-Path $PSScriptRoot 'launch-train.ps1') -Name $Name -Resume -RunnerHash $RunnerHash
    exit $LASTEXITCODE
}
if ($stop -notmatch 'candidate notebooks are not green') { [Console]::WriteLine("$Name stopped for another reason ($stop); a resume can't help"); exit 1 }
$red = @(Get-Content -LiteralPath (Join-Path $out 'notebooks-verdicts.txt') | Where-Object { $_ -notmatch '^pass ' } | ForEach-Object { ($_ -split '\s+')[1] })
$real = @($red | Where-Object { -not (Select-String -LiteralPath (Join-Path $out "$($_ -replace '[\\/]', '_').log") -Pattern 'Spiral cell timed out' -Quiet) })
if ($real.Count) { [Console]::WriteLine("$Name has $($real.Count) red notebooks that did not time out: $($real -join ', '); fix them first"); exit 1 }
[Console]::WriteLine("$Name stopped only on $($red.Count) timed-out notebooks; resuming")
& (Join-Path $PSScriptRoot 'launch-train.ps1') -Name $Name -Resume -RunnerHash $RunnerHash
