param([string]$Eoie = (Join-Path $PSScriptRoot '..\src\target\debug\eoie.exe'))
$ErrorActionPreference = 'Stop'
$marker = "eoie-ps-e2e-$PID"
$sleeper = Start-Process pwsh -ArgumentList @('-NoProfile', '-Command', "`$null = '$marker'; Start-Process pwsh -ArgumentList '-NoProfile','-Command','Start-Sleep 120' -NoNewWindow; Start-Sleep 120") -WindowStyle Hidden -PassThru
Start-Sleep -Milliseconds 1500
$failures = 0
function Check([string]$Name, [bool]$Condition, [string[]]$Output) {
    if ($Condition) { "PASS $Name" } else { $script:failures++; "FAIL $Name"; $Output | ForEach-Object { "  | $_" } }
}
try {
    $listed = @(& $Eoie ps $marker 2>&1 | ForEach-Object { "$_" })
    Check 'ps finds the marker process' (($listed -join "`n") -match "(?m)^$($sleeper.Id)\s" -and $listed[-1] -match '^ok  processes 1  ') $listed
    $tree = @(& $Eoie ps $marker --tree 2>&1 | ForEach-Object { "$_" })
    Check 'ps --tree adds the child indented' (($tree | Where-Object { $_ -match "^\d+\s+$($sleeper.Id)\s" -and $_ -match 'Start-Sleep 120' }).Count -ge 1) $tree
    $refused = @(& $Eoie kill $PID 2>&1 | ForEach-Object { "$_" })
    Check 'kill refuses the calling shell' ($LASTEXITCODE -eq 1 -and ($refused -join "`n") -match 'refused: that is a parent of this eoie') $refused
    $stopped = @(& $Eoie kill $sleeper.Id --tree 2>&1 | ForEach-Object { "$_" })
    Start-Sleep -Milliseconds 500
    $gone = -not (Get-Process -Id $sleeper.Id -ErrorAction SilentlyContinue)
    Check 'kill --tree stops the process and its child' ($LASTEXITCODE -eq 0 -and $gone -and ($stopped -join "`n") -match 'stopped \(tree: \d+ processes\)') $stopped
    $missing = @(& $Eoie kill 4294967 2>&1 | ForEach-Object { "$_" })
    Check 'kill of a missing pid fails' ($LASTEXITCODE -eq 1 -and ($missing -join "`n") -match 'stop 4294967  not found' -and $missing[-1] -match '^failed  stopped 0  failed 1  ') $missing
} finally {
    if (Get-Process -Id $sleeper.Id -ErrorAction SilentlyContinue) { Stop-Process -Id $sleeper.Id -Force }
}
"ps e2e: $(5 - $failures)/5 passed"
exit $failures
