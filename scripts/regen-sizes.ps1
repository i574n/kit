param([Parameter(Mandatory)][string[]] $Package, [int] $Limit = 1000)
$ErrorActionPreference = 'Stop'
$kitRoot = Split-Path $PSScriptRoot -Parent
$Package = @($Package | ForEach-Object { $_ -split ',' } | ForEach-Object { $_.Trim() } | Where-Object { $_ })
if (-not $env:KIT_SPIRAL_BUNDLE) { $env:KIT_SPIRAL_BUNDLE = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\spiral\apps\compiler\tmp')).Path }
$quoted = ($Package | ForEach-Object { "'$_'" }) -join ','
$text = (& pwsh -NoProfile -Command "& '$(Join-Path $kitRoot 'compiler-contracts\test-regeneration.ps1')' -KitRoot '$kitRoot' -Package $quoted -CargoCheck -KeepStaged" 2>&1 | Out-String)
$staged = [regex]::Match($text, 'Staged workspace: (\S+)').Groups[1].Value
if (-not $staged) { [Console]::WriteLine(($text -split "`n" | Select-Object -Last 15) -join "`n"); exit 2 }
$over = 0
foreach ($name in $Package) {
    $member = $name.Replace('-', '_')
    $emitted = Join-Path (Split-Path $staged) "outputs\$member.rs"
    $sources = if (Test-Path -LiteralPath $emitted) { @(Get-Item -LiteralPath $emitted) } else { @(Get-ChildItem -LiteralPath (Join-Path $staged "src\$member") -Filter *.rs -File -ErrorAction SilentlyContinue) }
    foreach ($rs in $sources) {
        $lines = @(Get-Content -LiteralPath $rs.FullName).Count
        $live = @(Get-ChildItem -LiteralPath (Join-Path $kitRoot "src\$member") -Filter *.rs -File -ErrorAction SilentlyContinue | Select-Object -First 1 -ExpandProperty FullName)
        $before = if ($live) { @(Get-Content -LiteralPath $live[0]).Count } else { 0 }
        $mark = if ($lines -gt $Limit) { $over++; 'OVER' } else { 'ok' }
        [Console]::WriteLine("$($mark.PadRight(4)) $name/$($rs.Name) $lines lines (tree has $before)")
    }
}
[Console]::WriteLine("regen sizes: $($Package.Count) packages, $over over $Limit; staged at $staged")
exit $(if ($over) { 1 } else { 0 })
