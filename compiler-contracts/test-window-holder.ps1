param([string] $Kit = 'C:\home\git\eoie\kit.exe')
$label = "kit probe: stopped holder drops its window $PID"
$start = & $Kit job start kit-window-holder-probe 300000 $Kit window run compile 2 --label $label -- pwsh -NoProfile -Command 'Start-Sleep -Seconds 90'
$id = [regex]::Match("$start", 'id=(\S+)').Groups[1].Value
$held = $false
foreach ($attempt in 1..40) { if ((& $Kit window list 2>&1) -match [regex]::Escape($label)) { $held = $true; break }; Start-Sleep -Milliseconds 500 }
& $Kit job stop $id | Out-Null
Start-Sleep -Seconds 3
$rows = @((& $Kit window list 2>&1) | Where-Object { $_ -match [regex]::Escape($label) })
"held before stop: $held; rows after stop: $($rows.Count)"
foreach ($row in $rows) { $window = ("$row" -split '\s+')[0]; & $Kit window release $window | Out-Null }
if ($held -and $rows.Count -eq 0) { 'PASS a stopped holder drops its window'; exit 0 } else { 'FAIL'; exit 1 }
