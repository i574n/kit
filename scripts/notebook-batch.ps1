param([Parameter(Mandatory)][string[]] $Module, [string] $Out = (Join-Path ([IO.Path]::GetTempPath()) 'kit-notebook-batch'))
$ErrorActionPreference = 'Stop'
$kitRoot = Split-Path $PSScriptRoot -Parent
$spiral = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\spiral')).Path
$runner = Join-Path $spiral 'apps\cells\spi\run_notebook.ps1'
$env:SPIRAL_LIB = Join-Path $spiral 'lib\spiral'
$env:SPIRAL_CELLS_LIB = $env:SPIRAL_LIB
$env:SPIRAL_CELLS_WARM = '0'; $env:SPIRAL_CELLS_MEMO = '0'; $env:SPIRAL_CELLS_COMPILER = 'process'
[void][IO.Directory]::CreateDirectory($Out)
$env:MIX_BUILD_PATH = Join-Path $Out 'mix'
$Module = @($Module | ForEach-Object { $_ -split ',' } | ForEach-Object { $_.Trim() } | Where-Object { $_ })
$red = 0
foreach ($relative in $Module) {
    $path = Join-Path $kitRoot $relative
    $name = $relative -replace '[\\/]', '_'
    $text = (& pwsh -NoProfile -File $runner --path $path --output-path (Join-Path $Out "$name.ipynb") --no-html 2>&1 | Out-String)
    $total = [regex]::Match($text, 'spiral-tests total (\{[^\r\n]*\})').Groups[1].Value
    $verdict = if ($total -match '"failed":0,' -and $total -notmatch '"executed":0,') { 'pass' } else { $red++; 'RED' }
    [Console]::WriteLine("$verdict $relative $total")
    if ($verdict -eq 'RED') { [Console]::WriteLine(([regex]::Matches($text, '(ETY-\w+[^\r\n]{0,200}|First error: [^\r\n]{0,240}|timed out[^\r\n]{0,80})') | Select-Object -First 3 | ForEach-Object Value) -join "`n") }
}
[Console]::WriteLine("notebook batch: $($Module.Count - $red)/$($Module.Count) pass")
exit $(if ($red) { 1 } else { 0 })
