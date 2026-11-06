. $PSScriptRoot/workspace.ps1
$compilerRoot = Resolve-KitSpiralBundle
. (@((Join-Path (Split-Path $compilerRoot) 'scripts/env.ps1'), (Join-Path $compilerRoot 'scripts/env.ps1')) | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1)
