param([string]$Eoie = (Join-Path $PSScriptRoot '..\src\target\debug\eoie.exe'))
$ErrorActionPreference = 'Stop'
$env:EOIE_JOBS_HOME = Join-Path ([IO.Path]::GetTempPath()) "eoie-timeout-tree-$PID"
$marker = "eoie-timeout-tree-$PID"
$started = & $Eoie job start timeout-tree 3000 pwsh -NoProfile -Command "`$null = '$marker'; Start-Process pwsh -ArgumentList '-NoProfile','-Command','`$null = ''$marker''; Start-Sleep 60' -NoNewWindow; Start-Sleep 60"
$id = [regex]::Match("$started", 'id=(\S+)').Groups[1].Value
$waited = & $Eoie job wait $id 30000
$code = $LASTEXITCODE
Start-Sleep -Milliseconds 300
$survivors = @(Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -match [regex]::Escape($marker) -and $_.ProcessId -ne $PID })
$failures = 0
if ($code -eq 124 -and "$waited" -match 'state=timed-out' -and "$waited" -match 'detail="wall-clock timeout; process tree stopped"') { 'PASS timeout reports the stopped tree' } else { $failures++; "FAIL timeout report (exit $code): $waited" }
if ($survivors.Count -eq 0) { 'PASS no process of the job tree survives' } else { $failures++; "FAIL $($survivors.Count) survivors: $($survivors.ProcessId -join ',')"; $survivors | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue } }
"timeout tree: $(2 - $failures)/2 passed"
exit $failures
