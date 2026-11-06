param([string]$Kit = (Join-Path $PSScriptRoot '..\src\target\debug\kit.exe'))
$ErrorActionPreference = 'Stop'
$env:KIT_JOBS_HOME = Join-Path ([IO.Path]::GetTempPath()) "kit-hidden-window-$PID"
$marker = "kit-hidden-window-$PID"
$started = & $Kit job start hidden-window 6000 pwsh -NoProfile -Command "`$null = '$marker'; Start-Sleep 30"
$id = [regex]::Match("$started", 'id=(\S+)').Groups[1].Value
$deadline = [DateTime]::UtcNow.AddSeconds(5)
$child = $null
while (-not $child -and [DateTime]::UtcNow -lt $deadline) {
    $child = Get-CimInstance Win32_Process -Filter "Name='pwsh.exe'" | Where-Object { $_.CommandLine -match [regex]::Escape($marker) -and $_.ProcessId -ne $PID } | Select-Object -First 1
    if (-not $child) { Start-Sleep -Milliseconds 100 }
}
$failures = 0
if (-not $child) { $failures++; 'FAIL the job child never appeared' }
else {
    $deadline = [DateTime]::UtcNow.AddMilliseconds(1500)
    $handle = 0
    while ($handle -eq 0 -and [DateTime]::UtcNow -lt $deadline) {
        $process = Get-Process -Id $child.ProcessId -ErrorAction SilentlyContinue
        if ($process) { $process.Refresh(); $handle = $process.MainWindowHandle.ToInt64() }
        Start-Sleep -Milliseconds 100
    }
    if ($handle -eq 0) { 'PASS the job child has no visible window' } else { $failures++; "FAIL the job child shows window $handle" }
}
$waited = & $Kit job wait $id 30000
if ($LASTEXITCODE -eq 124) { 'PASS the job still stops on its timeout' } else { $failures++; "FAIL job wait exit ${LASTEXITCODE}: $waited" }
"hidden window: $(2 - $failures)/2 passed"
exit $failures
