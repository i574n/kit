param([string] $Project = 'C:\home\remote-stage\iggy-m2', [string] $Distro = 'Ubuntu')
$ErrorActionPreference = 'Stop'
$log = Join-Path $Project 'm4.log'
function Say([string] $text) { $line = "$(Get-Date -Format 'HH:mm:ss') $text"; $line; Add-Content -Path $log -Value $line }
Set-Location $Project
$encoded = (wsl.exe -d $Distro -u root -- sh -c "k3s kubectl -n iggy get secret iggy-root -o jsonpath='{.data.password}'" | Out-String).Trim()
if (-not $encoded) { Say 'STOP: could not read secret iggy/iggy-root'; exit 1 }
$env:IGGY_PASSWORD = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($encoded))
$env:IGGY_ADDRESS = '127.0.0.1:8090'
cargo build --release --bins *> (Join-Path $Project 'build-m4.log')
if ($LASTEXITCODE -ne 0) { Get-Content (Join-Path $Project 'build-m4.log') -Tail 20 | ForEach-Object { Say "  build: $_" }; Say 'STOP: build failed'; exit 1 }
Say 'publish and follow built'
$sample = Join-Path $Project 'sample-job.ps1'
@'
function Say([string] $text) { "$(Get-Date -Format 'HH:mm:ss') $text" }
Say 'fix: ok  files 1  replacements 1'
Say 'regen of kit_sample exit 0; generated code checked'
Say 'debug build exit 0; candidate ABCDEF'
Start-Sleep -Seconds 2
Say 'positive control src\x.spi (a pure model notebook): exit 0, executed 2, passed 2, unsupported 0'
Say 'notebooks done: 3/3 pass'
Say 'DONE'
'@ | Set-Content $sample
$labelOk = "m4-sample-$(Get-Date -Format HHmmss)"
$follower = Start-Process -FilePath (Join-Path $Project 'target\release\follow.exe') -ArgumentList $labelOk, '5' -RedirectStandardOutput (Join-Path $Project 'follow-ok.txt') -RedirectStandardError (Join-Path $Project 'follow-ok.err') -PassThru -NoNewWindow
Start-Sleep -Seconds 2
& (Join-Path $Project 'target\release\publish.exe') $labelOk $sample *> (Join-Path $Project 'publish-ok.txt')
$publishExit = $LASTEXITCODE
$watch = [Diagnostics.Stopwatch]::StartNew()
$finished = $follower.WaitForExit(30000)
Say "sample job: publish exit $publishExit; follower finished $finished in $($watch.ElapsedMilliseconds) ms after the job ended, exit $(if ($finished) { $follower.ExitCode } else { '-' })"
Get-Content (Join-Path $Project 'follow-ok.txt') | ForEach-Object { Say "  follow: $_" }
$stopSample = Join-Path $Project 'sample-stop.ps1'
"function Say([string] `$text) { `"`$(Get-Date -Format 'HH:mm:ss') `$text`" }`nSay 'regen of kit_sample exit 1; failed'`nSay 'STOP: regen failed'`nexit 1" | Set-Content $stopSample
$labelStop = "m4-stop-$(Get-Date -Format HHmmss)"
& (Join-Path $Project 'target\release\publish.exe') $labelStop $stopSample *> (Join-Path $Project 'publish-stop.txt')
$stopFollow = @(& (Join-Path $Project 'target\release\follow.exe') $labelStop 1 2>&1 | ForEach-Object { "$_" })
Say "stop sample: follower exit $LASTEXITCODE (2 means JobStop)"
$stopFollow | ForEach-Object { Say "  follow: $_" }
Say 'DONE'
