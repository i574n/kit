param([string] $Eoie = 'C:\home\git\eoie\eoie.exe')
$label = "eoie probe: stopped holder drops its window $PID"
$start = & $Eoie job start eoie-window-holder-probe 300000 $Eoie window run compile 2 --label $label -- pwsh -NoProfile -Command 'Start-Sleep -Seconds 90'
$id = [regex]::Match("$start", 'id=(\S+)').Groups[1].Value
$held = $false
foreach ($attempt in 1..40) { if ((& $Eoie window list 2>&1) -match [regex]::Escape($label)) { $held = $true; break }; Start-Sleep -Milliseconds 500 }
& $Eoie job stop $id | Out-Null
Start-Sleep -Seconds 3
$rows = @((& $Eoie window list 2>&1) | Where-Object { $_ -match [regex]::Escape($label) })
"held before stop: $held; rows after stop: $($rows.Count)"
foreach ($row in $rows) { $window = ("$row" -split '\s+')[0]; & $Eoie window release $window | Out-Null }
if ($held -and $rows.Count -eq 0) { 'PASS a stopped holder drops its window'; exit 0 } else { 'FAIL'; exit 1 }
