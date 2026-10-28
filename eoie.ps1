$ErrorActionPreference = 'Stop'
$binaryName = if ($env:OS -eq 'Windows_NT') { 'kit.exe' } else { 'kit' }
$binary = Join-Path $PSScriptRoot $binaryName
if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    & (Join-Path $PSScriptRoot 'build.ps1')
}
& $binary @args
exit $LASTEXITCODE
