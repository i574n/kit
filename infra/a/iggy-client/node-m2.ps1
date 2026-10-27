param([string] $Project = 'C:\home\remote-stage\iggy-m2', [int] $Count = 10000, [string] $Distro = 'Ubuntu')
$ErrorActionPreference = 'Stop'
$log = Join-Path $Project 'm2.log'
function Say([string] $text) { $line = "$(Get-Date -Format 'HH:mm:ss') $text"; $line; Add-Content -Path $log -Value $line }
Set-Location $Project
$encoded = (wsl.exe -d $Distro -u root -- sh -c "k3s kubectl -n iggy get secret iggy-root -o jsonpath='{.data.password}'" | Out-String).Trim()
if (-not $encoded) { Say 'STOP: could not read secret iggy/iggy-root'; exit 1 }
$env:IGGY_PASSWORD = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($encoded))
$env:IGGY_ADDRESS = '127.0.0.1:8090'
Say 'credentials loaded from secret iggy/iggy-root (not printed)'
cargo test --release *> (Join-Path $Project 'test.log')
$testExit = $LASTEXITCODE
Say "wire tests exit ${testExit}: $((Select-String -Path (Join-Path $Project 'test.log') -Pattern 'test result:' | Select-Object -First 1).Line)"
if ($testExit -ne 0) { Get-Content (Join-Path $Project 'test.log') -Tail 15 | ForEach-Object { Say "  test: $_" }; Say 'STOP: tests failed'; exit 1 }
cargo build --release *> (Join-Path $Project 'build.log')
if ($LASTEXITCODE -ne 0) { Get-Content (Join-Path $Project 'build.log') -Tail 15 | ForEach-Object { Say "  build: $_" }; Say 'STOP: build failed'; exit 1 }
$result = @(& (Join-Path $Project 'target\release\iggy-m2.exe') $Count 2>&1 | ForEach-Object { "$_" })
$runExit = $LASTEXITCODE
$result | ForEach-Object { Say "run: $_" }
Say "round trip exit $runExit"
Say 'DONE'
