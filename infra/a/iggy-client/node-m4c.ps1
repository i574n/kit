param([string] $Project = 'C:\home\remote-stage\iggy-m2', [string] $Distro = 'Ubuntu')
$ErrorActionPreference = 'Stop'
$log = Join-Path $Project 'm4c.log'
function Say([string] $text) { $line = "$(Get-Date -Format 'HH:mm:ss') $text"; $line; Add-Content -Path $log -Value $line }
$encoded = (wsl.exe -d $Distro -u root -- sh -c "k3s kubectl -n iggy get secret iggy-root -o jsonpath='{.data.password}'" | Out-String).Trim()
if (-not $encoded) { Say 'STOP: could not read secret iggy/iggy-root'; exit 1 }
$env:IGGY_PASSWORD = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($encoded))
$env:IGGY_ADDRESS = '127.0.0.1:8090'
$label = "m4c-arity-$(Get-Date -Format HHmmss)"
$stage = 'C:\home\remote-stage\renameproof'
$outName = 'm4c'
if (Test-Path (Join-Path $stage $outName)) { Rename-Item (Join-Path $stage $outName) "$outName-old-$(Get-Date -Format HHmmss)" }
$follower = Start-Process -FilePath (Join-Path $Project 'target\release\follow.exe') -ArgumentList $label, '60' -RedirectStandardOutput (Join-Path $Project 'm4c-follow.txt') -RedirectStandardError (Join-Path $Project 'm4c-follow.err') -PassThru -NoNewWindow
Say "follower started for $label"
& (Join-Path $Project 'target\release\publish.exe') $label (Join-Path $stage 'scripts\node-candidate.ps1') -Mirror C:\home\remote\arity\eoie -Fixes none -RegenCrates 'kit_adoption,kit_agile_mutation' -OutName $outName -Only 'agile|notes' *> (Join-Path $Project 'm4c-publish.txt')
$jobEnded = Get-Date
Say "job ended, publish exit $LASTEXITCODE"
$finished = $follower.WaitForExit(30000)
Say "follower finished $finished, exit $(if ($finished) { $follower.ExitCode } else { '-' }), $([math]::Round(((Get-Date) - $jobEnded).TotalMilliseconds)) ms after the job ended"
$jobLog = Get-Content (Join-Path $stage "$outName\candidate.log")
$verdictLines = @($jobLog | Where-Object { $text = $_.Substring([Math]::Min(9, $_.Length)).Trim(); -not $text.StartsWith('STOP') -and ($text.StartsWith('notebooks done') -or $text.StartsWith('positive control') -or $text -match '\bexit -?\d+') })
$events = @(Get-Content (Join-Path $Project 'm4c-follow.txt') | Where-Object { $_ -match '^step ' })
Say "job log verdict lines: $($verdictLines.Count); follower step events: $($events.Count)"
$mismatch = 0
for ($i = 0; $i -lt [Math]::Max($verdictLines.Count, $events.Count); $i++) {
    $want = if ($i -lt $verdictLines.Count) { $verdictLines[$i].Substring(9).Trim() } else { '(none)' }
    $got = if ($i -lt $events.Count) { ($events[$i] -split ': ', 2)[1] } else { '(none)' }
    if ($want -ne $got) { $mismatch++; Say "  mismatch $i want '$want' got '$got'" }
}
Say "M4c: $(if ($mismatch -eq 0 -and $verdictLines.Count -gt 0) { 'PASS every verdict line has exactly one event, same order' } else { "FAIL $mismatch mismatches" })"
Get-Content (Join-Path $Project 'm4c-follow.txt') | Select-Object -Last 3 | ForEach-Object { Say "  follow: $_" }
Say 'DONE'
