param([Parameter(Mandatory)][string[]] $Package)
$ErrorActionPreference = 'Stop'
$kitRoot = Split-Path $PSScriptRoot -Parent
$Package = @($Package | ForEach-Object { $_ -split ',' } | ForEach-Object { $_.Trim() } | Where-Object { $_ })
if (-not $env:KIT_SPIRAL_BUNDLE) { $env:KIT_SPIRAL_BUNDLE = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\spiral\apps\compiler\tmp')).Path }
$quoted = ($Package | ForEach-Object { "'$_'" }) -join ','
$regen = (& pwsh -NoProfile -Command "& '$(Join-Path $kitRoot 'compiler-contracts\test-regeneration.ps1')' -KitRoot '$kitRoot' -Package $quoted -CargoCheck -Publish" 2>&1 | Out-String)
if ($regen -notmatch 'KIT regeneration passed') { [Console]::WriteLine(($regen -split "`n" | Select-Object -Last 20) -join "`n"); exit 2 }
[Console]::WriteLine(($regen -split "`n" | Where-Object { $_ -match 'Published|passed' }) -join "`n")
Push-Location (Join-Path $kitRoot 'src')
try { $build = (& cargo build --locked -p kit-cli 2>&1 | Out-String) } finally { Pop-Location }
if ($LASTEXITCODE -ne 0) { [Console]::WriteLine(($build -split "`n" | Select-Object -Last 20) -join "`n"); exit 3 }
$candidate = Join-Path $kitRoot 'src\target\debug\kit.exe'
[Console]::WriteLine("candidate $candidate $((Get-FileHash $candidate).Hash.Substring(0, 8))")
