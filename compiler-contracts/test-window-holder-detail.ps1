$e = 'C:\home\git\eoie\eoie.exe'
$label = "eoie probe detail $PID"
$start = & $e job start eoie-window-holder-probe-detail 300000 $e window run compile 2 --label $label -- pwsh -NoProfile -Command 'Start-Sleep -Seconds 90'
$id = [regex]::Match("$start", 'id=(\S+)').Groups[1].Value
foreach ($attempt in 1..40) { if ((& $e window list 2>&1) -match [regex]::Escape($label)) { break }; Start-Sleep -Milliseconds 500 }
$row = @((& $e window list 2>&1) | Where-Object { $_ -match [regex]::Escape($label) })[0]
$window = ("$row" -split '\s+')[0]
$holder = [int](($window -split '-')[2])
"row before stop: $row"
"holder pid from window id: $holder alive=$([bool](Get-Process -Id $holder -ErrorAction SilentlyContinue))"
$file = Get-ChildItem "$env:LOCALAPPDATA\eoie" -Recurse -Filter "$window*" -ErrorAction SilentlyContinue | Select-Object -First 1
"window file: $($file.FullName)"
if ($file) { "window record: $((Get-Content -Raw $file.FullName) -replace "`n", ' | ')" }
"stop: $((& $e job stop $id 2>&1) -join ' ')"
Start-Sleep -Seconds 3
"holder alive after stop: $([bool](Get-Process -Id $holder -ErrorAction SilentlyContinue))"
$after = @((& $e window list 2>&1) | Where-Object { $_ -match [regex]::Escape($label) })
"rows after stop: $($after.Count) $($after -join ' / ')"
$tree = Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -match [regex]::Escape($label) } | ForEach-Object { "$($_.ProcessId) $($_.Name)" }
"processes still carrying the label: $($tree -join ', ')"
foreach ($line in $after) { & $e window release (("$line" -split '\s+')[0]) | Out-Null }
