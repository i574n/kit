param([string]$Kit = (Join-Path $PSScriptRoot '..\src\target\debug\kit.exe'))
$ErrorActionPreference = 'Stop'
$Kit = (Resolve-Path $Kit).Path
$env:KIT_JOBS_HOME = Join-Path ([IO.Path]::GetTempPath()) "kit-orphan-wait-$PID"
$started = & $Kit job start orphan-wait 90000 pwsh -NoProfile -Command 'Start-Sleep 60'
$id = [regex]::Match("$started", 'id=(\S+)').Groups[1].Value
$shell = Start-Process pwsh -ArgumentList '-NoProfile', '-Command', "& '$Kit' job wait $id 120000" -WindowStyle Hidden -PassThru
$waiters = { @(Get-CimInstance Win32_Process -Filter "CommandLine LIKE '%job wait $id%'") }
$deadline = [DateTime]::UtcNow.AddSeconds(90)
while ((& $waiters).Count -lt 2 -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
$before = (& $waiters).Count
$kitWaiters = @(& $waiters | Where-Object { $_.Name -like 'kit*' }).Count
Stop-Process -Id $shell.Id -Force
$deadline = [DateTime]::UtcNow.AddSeconds(3)
while ((& $waiters).Count -gt 0 -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
$after = @(& $waiters)
$failures = 0
if ($before -ge 1) { "PASS the waiter started ($before processes)" } else { $failures++; 'FAIL the waiter never started' }
if ($kitWaiters -eq 1) { 'PASS one kit process per waiter (no relaunched copy)' } else { $failures++; "FAIL $kitWaiters kit processes for one job wait" }
if ($after.Count -eq 0) { 'PASS job wait ends when its shell is killed' } else { $failures++; "FAIL $($after.Count) job wait processes outlived their shell: $($after.ProcessId -join ',')"; $after | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue } }
$supervisor = [regex]::Match("$started", 'supervisor_pid=(\d+)').Groups[1].Value
if ($supervisor) { & $Kit kill $supervisor --tree | Out-Null }
Remove-Item -Recurse -Force $env:KIT_JOBS_HOME -ErrorAction SilentlyContinue
"orphan wait: $(3 - $failures)/3 passed"
exit $failures
