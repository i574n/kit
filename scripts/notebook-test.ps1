param([Parameter(Mandatory)][string] $Module, [string] $Bin = '', [int] $TimeoutMs = 1800000)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'user-env.ps1')
$kit = Join-Path $env:LOCALAPPDATA 'kit\bin\kit.exe'
$path = (Resolve-Path -LiteralPath $Module).Path
$runner = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..\spiral\apps\cells\spi\run_notebook.ps1')).Path
$label = 'kit-nb-' + [IO.Path]::GetFileNameWithoutExtension($path)
$out = Join-Path $env:TEMP $label
[void][IO.Directory]::CreateDirectory($out)
$env:SPIRAL_LIB = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..\spiral\lib\spiral')).Path
$env:SPIRAL_CELLS_WARM = '0'; $env:SPIRAL_CELLS_MEMO = '0'; $env:SPIRAL_CELLS_COMPILER = 'process'
$env:MIX_BUILD_PATH = Join-Path $out 'mix'
if ($Bin) { $env:KIT_BIN_UNDER_TEST = (Resolve-Path -LiteralPath $Bin).Path }
$started = & $kit job run --compile $label $TimeoutMs pwsh -NoProfile -File $runner --path $path --output-path (Join-Path $out 'notebook.ipynb') --no-html 2>&1
$id = [regex]::Match("$started", 'id=(\S+)').Groups[1].Value
if (-not $id) { "$started"; exit 2 }
$stdout = Get-Content (Join-Path $env:LOCALAPPDATA "kit\jobs\$id\stdout.log") -Raw
$total = [regex]::Match($stdout, 'spiral-tests total (\{[^\r\n]*\})').Groups[1].Value
"$id $([IO.Path]::GetFileName($path)) $total"
$firstError = [regex]::Match($stdout, 'First error: [^\r\n]{0,300}').Value
if ($firstError) { $firstError }
if (-not $total -or $total -notmatch '"failed":0,') { exit 1 }