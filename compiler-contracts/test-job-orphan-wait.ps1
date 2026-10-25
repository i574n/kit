param([string]$Eoie = (Join-Path $PSScriptRoot '..\src\target\debug\eoie.exe'))
$ErrorActionPreference = 'Stop'
$Eoie = (Resolve-Path $Eoie).Path
$env:EOIE_JOBS_HOME = Join-Path ([IO.Path]::GetTempPath()) "eoie-orphan-wait-$PID"
$started = & $Eoie job start orphan-wait 90000 pwsh -NoProfile -Command 'Start-Sleep 60'
$id = [regex]::Match("$started", 'id=(\S+)').Groups[1].Value
$shell = Start-Process pwsh -ArgumentList '-NoProfile', '-Command', "& '$Eoie' job wait $id 120000" -WindowStyle Hidden -PassThru
$waiters = { @(Get-CimInstance Win32_Process -Filter "CommandLine LIKE '%job wait $id%'") }
$deadline = [DateTime]::UtcNow.AddSeconds(90)
while ((& $waiters).Count -lt 2 -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
$before = (& $waiters).Count
$eoieWaiters = @(& $waiters | Where-Object { $_.Name -like 'eoie*' }).Count
Stop-Process -Id $shell.Id -Force
$deadline = [DateTime]::UtcNow.AddSeconds(3)
while ((& $waiters).Count -gt 0 -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
$after = @(& $waiters)
$failures = 0
if ($before -ge 1) { "PASS the waiter started ($before processes)" } else { $failures++; 'FAIL the waiter never started' }
if ($eoieWaiters -eq 1) { 'PASS one eoie process per waiter (no relaunched copy)' } else { $failures++; "FAIL $eoieWaiters eoie processes for one job wait" }
if ($after.Count -eq 0) { 'PASS job wait ends when its shell is killed' } else { $failures++; "FAIL $($after.Count) job wait processes outlived their shell: $($after.ProcessId -join ',')"; $after | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue } }
$supervisor = [regex]::Match("$started", 'supervisor_pid=(\d+)').Groups[1].Value
if ($supervisor) { & $Eoie kill $supervisor --tree | Out-Null }
Remove-Item -Recurse -Force $env:EOIE_JOBS_HOME -ErrorAction SilentlyContinue
"orphan wait: $(3 - $failures)/3 passed"
exit $failures
