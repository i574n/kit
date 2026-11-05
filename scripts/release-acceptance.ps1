param([Parameter(Mandatory)][string] $Candidate, [string] $Work = (Join-Path $env:TEMP ('kit-cand-' + [guid]::NewGuid().ToString('N').Substring(0, 8))))
$ErrorActionPreference = 'Continue'
$installed = Join-Path $env:LOCALAPPDATA 'kit\bin\kit.exe'
$rows = [Collections.Generic.List[string]]::new()
function Row([string] $Name, [bool] $Pass, [string] $Evidence) {
    $line = '{0,-5} {1,-44} {2}' -f $(if ($Pass) { 'PASS' } else { 'FAIL' }), $Name, (($Evidence -replace '\s+', ' ').Trim())
    $rows.Add($line); [Console]::WriteLine($line)
}
function Clip([string] $Text) { $Text.Substring(0, [Math]::Min(150, $Text.Length)) }
[void][IO.Directory]::CreateDirectory($Work)
"candidate $((Get-FileHash $Candidate).Hash.Substring(0, 8)) installed $((Get-FileHash $installed).Hash.Substring(0, 8))"

$env:CLAUDECODE = '1'
Push-Location $Work
[void][IO.Directory]::CreateDirectory((Join-Path $Work 'repo')); Set-Content (Join-Path $Work 'repo\a.txt') 'one'
$made = (& $Candidate do . 'git init repo; git commit repo all "init" at 2026-10-01T10:00:00Z' 2>&1 | Out-String)
Row 'agent commits a repo its program created' ($LASTEXITCODE -eq 0 -and $made -match 'committed 1') (Clip $made)
Set-Content (Join-Path $Work 'repo\a.txt') 'change'
$refused = (& $Candidate do . 'git commit repo all "agent change"' 2>&1 | Out-String)
Row 'agent is refused on a repo it did not create' ($LASTEXITCODE -eq 1 -and $refused -match 'refused: an agent session') (Clip $refused)
$log = (git -C (Join-Path $Work 'repo') log --oneline 2>&1 | Out-String)
Row 'the refused commit wrote nothing' (@($log.Trim() -split "`n").Count -eq 1) (Clip $log)
Remove-Item Env:CLAUDECODE
$saved = $env:AI_AGENT; Remove-Item Env:AI_AGENT -ErrorAction SilentlyContinue
$person = (& $Candidate do . 'git commit repo all "user change" at 2026-10-02T10:00:00Z' 2>&1 | Out-String)
Row 'a non-agent run still commits' ($LASTEXITCODE -eq 0 -and $person -match 'committed 1') (Clip $person)
if ($saved) { $env:AI_AGENT = $saved }
Pop-Location

foreach ($repo in 'C:/home/git/kit', 'C:/home/git/i574n.github', 'C:/home/git/spiral') {
    $program = "git status $repo group by state"
    foreach ($attempt in 1..3) {
        $a = (& $Candidate do C:\ $program 2>&1 | Where-Object { $_ -notmatch '^\S+\s+repos \d+' } | Out-String)
        $b = (& $installed do C:\ $program 2>&1 | Where-Object { $_ -notmatch '^\S+\s+repos \d+' } | Out-String)
        $tailA = (& $Candidate do C:\ $program 2>&1 | Select-Object -Last 1) -replace '\s+\d+ms\s+\S+$', ''
        $tailB = (& $installed do C:\ $program 2>&1 | Select-Object -Last 1) -replace '\s+\d+ms\s+\S+$', ''
        if ($a -eq $b -and $tailA -eq $tailB) { break }
    }
    Row "git status rows equal installed: $repo" ($a -eq $b -and $tailA -eq $tailB) (Clip "$tailA | rows $(@($a -split "`n").Count) | attempt $attempt")
}

$appendRoot = Join-Path $Work 'append'
[void][IO.Directory]::CreateDirectory($appendRoot)
$appended = (& $Candidate do $appendRoot 'append newpkg/sub/package.spiproj "hello"' 2>&1 | Out-String)
Row 'append creates parent folders' ($LASTEXITCODE -eq 0 -and (Test-Path (Join-Path $appendRoot 'newpkg\sub\package.spiproj'))) (Clip $appended)
[IO.File]::WriteAllText((Join-Path $appendRoot 'newpkg\a.txt'), 'literal')
$copied = (& $Candidate do $appendRoot 'copy newpkg/a.txt to out' 2>&1 | Out-String)
Row 'copy with a literal file source copies the file' ($LASTEXITCODE -eq 0 -and (Test-Path (Join-Path $appendRoot 'out\a.txt'))) (Clip $copied)

$lint = @(& $Candidate lint macros C:\home\git\spiral\lib\spiral 2>&1)
$flow = @($lint | Where-Object { $_ -match '^\S+\.spi:\d+\s+\S+\s+flow\s' }).Count
Row 'lint macros on lib: 0 flow rows' ($flow -eq 0 -and $lint[-1] -match 'violations') (Clip $lint[-1])
$kitLint = @(& $Candidate lint macros C:\home\git\kit\src --baseline C:\home\git\kit\standards\lint-macros.spi 2>&1)
Row 'lint macros on kit: no new rows past baseline' ("$($kitLint[-1])" -match '\bnew 0\b') (Clip "$($kitLint[-1])")
$growth = @(& $Candidate lint growth C:/home/git/kit standards/lint-macros.spi C:/home/git/kit src 2>&1)
Row 'kit lint baseline grows only for code at HEAD' ($LASTEXITCODE -eq 0 -and "$($growth[-1])" -match '^kit lint growth ok') (Clip "$($growth[-1])")
$libGrowth = @(& $Candidate lint growth C:/home/git/kit standards/lint-macros-lib.spi C:/home/git/spiral lib/spiral 2>&1)
Row 'lib lint baseline grows only for code at HEAD' ($LASTEXITCODE -eq 0 -and "$($libGrowth[-1])" -match '^kit lint growth ok') (Clip "$($libGrowth[-1])")

$backlog = Join-Path $Work 'backlog'
& $installed agile init $backlog *> $null
& $installed agile add $backlog PROBE TaskKind P2 D2 'probe item' 'probe' *> $null
$text = "quote `" and backslash \ and `$true`nsecond line"
$noted = ($text | & $Candidate agile note $backlog PROBE - --as kit 2>&1 | Out-String)
$shown = (& $installed agile show $backlog PROBE 2>&1 | Out-String)
Row 'agile note reads stdin' ($noted -match 'note=1' -and $shown.Contains('quote " and backslash \ and $true') -and $shown.Contains('second line')) (Clip $noted)

& $installed agile add $backlog THEIRS TaskKind P0 D1 'theirs' 'kit,user-batch' *> $null
$next = (& $Candidate agile next $backlog 2>&1 | Out-String)
Row 'agile next hides user-batch items' ($next -match 'PROBE' -and $next -notmatch 'THEIRS') (Clip ($next -split "`n" | Select-Object -Last 2 | Out-String))

$memory = Join-Path $Work 'memory'
Copy-Item 'C:\Users\i574n\.claude\projects\C--home-git\memory' $memory -Recurse
$realIndex = Get-Content (Join-Path $memory 'MEMORY.md') -Raw
$refused = (& $Candidate memory save $memory feedback probe-rule 'a probe' 'No reasons.' 2>&1 | Out-String)
Row 'memory save refuses feedback without reasons' ($LASTEXITCODE -eq 2 -and $refused -match 'needs \*\*Why') (Clip $refused)
$saved = ("Rule.`n`n**Why:** probe.`n`n**How to apply:** tests." | & $Candidate memory save $memory feedback probe-rule 'a probe' - 2>&1 | Out-String)
Row 'memory save writes file and index line' ($LASTEXITCODE -eq 0 -and (Test-Path (Join-Path $memory 'probe-rule.md'))) (Clip $saved)
$edited = (& $Candidate memory edit $memory probe-rule description 'a better probe' 2>&1 | Out-String)
$hook = (Get-Content (Join-Path $memory 'MEMORY.md') | Select-Object -Last 1)
Row 'memory edit updates the description hook' ($LASTEXITCODE -eq 0 -and $hook -match 'a better probe$') (Clip "$edited $hook")
$forgot = (& $Candidate memory forget $memory probe-rule 2>&1 | Out-String)
$restored = (Get-Content (Join-Path $memory 'MEMORY.md') -Raw) -eq $realIndex
Row 'memory forget archives and restores the index' ($restored -and (Test-Path (Join-Path $memory 'archive\probe-rule.md'))) (Clip $forgot)
$memoryData = Join-Path $Work 'memory.spi'
$imported = (& $Candidate memory import $memory $memoryData 2>&1 | Out-String)
$recalled = @(& $Candidate memory recall $memoryData 'stopped session wake heartbeat' --limit 3 2>&1)
Row 'memory import round-trips and recall ranks by words' ($imported -match 'index_identical=yes' -and "$($recalled[0])" -match 'idle-needs-wake-source') (Clip "$imported $($recalled[0])")

$tsv = @(& $Candidate lint macros C:\home\git\spiral\lib\spiral --tsv 2>&1)
$wellFormed = @($tsv | Select-Object -Skip 1 -SkipLast 1 | Where-Object { ($_ -split "`t").Count -eq 5 }).Count
Row 'lint --tsv rows have 5 fields' ($tsv[0] -eq "file`tline`tname`tclass`ttext" -and $wellFormed -eq $tsv.Count - 2) (Clip "rows $wellFormed")
$shadowsA = (& $installed lint shadows C:\home\git\kit\src 2>&1 | Select-Object -Last 1) -replace '\s+\d+ms.*', ''
$shadowsB = (& $Candidate lint shadows C:\home\git\kit\src 2>&1 | Select-Object -Last 1) -replace '\s+\d+ms.*', ''
Row 'lint shadows walks the same files' ($shadowsA -eq $shadowsB) (Clip $shadowsB)
$opensRoot = Join-Path $Work 'opens'
[void][IO.Directory]::CreateDirectory((Join-Path $opensRoot 'pkg'))
[IO.File]::WriteAllText((Join-Path $opensRoot 'pkg\b.spi'), ":= helper = 2`n")
[IO.File]::WriteAllText((Join-Path $opensRoot 'pkg\a.spi'), ":< pkg.b`n:= helper = 1`n")
$opens = (& $Candidate lint shadows $opensRoot --opens 2>&1 | Out-String)
Row 'lint shadows --opens names a shadowed opened member' ($opens -match 'a\.spi:2  helper  open-shadow  from pkg\.b') (Clip $opens)

$kernel = @(& $Candidate ps 'Registry' 2>&1) | Where-Object { $_ -match '^\s*\d+\s+4\s' } | Select-Object -First 1
Row 'ps keeps the kernel as parent' ([bool]$kernel) (Clip "$kernel")

$tables = Join-Path $Work 'tables'
[void][IO.Directory]::CreateDirectory($tables)
[IO.File]::WriteAllText((Join-Path $tables 'median.tsv'), "name`tgroup`tms`na`ta`t10`nb`ta`t20`nc`ta`t30`nd`tb`t1`ne`tb`t4`nf`tb`t5`ng`tb`t9`n")
$median = (& $Candidate table query (Join-Path $tables 'median.tsv') --group group --median ms 2>&1 | Out-String)
Row 'table query --median' ($median -match "median_ms" -and $median -match "4`tb`t4.5" -and $median -match "3`ta`t20") (Clip $median)

[IO.File]::WriteAllText((Join-Path $tables 'zone.txt'), "10,10 protection-zone`n30,30 protection-zone`n99,99 floor-up`n")
[IO.File]::WriteAllText((Join-Path $tables 'model.trace'), "== run 1`nseen at 10,10: rat 20,20 rat 30,30`nseen at 11,11:`n== run 2`nseen at 50,50: rat 60,60`n")
$zone = (Join-Path $tables 'zone.txt') + ':^(?<value>\d+,\d+) protection-zone$'
$counted = (& $Candidate table sequences (Join-Path $tables 'model.trace') --section-on '^== (?<section>.*)$' --count 'samples=seen at ' `
    --count 'knight=seen at (?<value>\d+,\d+):' --in-set "knight=$zone" --count 'monsters= (?<value>\d+,\d+)' `
    --count-in 'monsters=seen at \d+,\d+:(?<rest>.*)$' --in-set "monsters=$zone" 2>&1 | Out-String)
Row 'table sequences --count/--in-set' ($counted -match "run 1`t2`t1`t1" -and $counted -match "run 2`t1`t0`t0") (Clip $counted)

$health = (& $Candidate machine health 2>&1 | Select-Object -Last 1 | Out-String)
Row 'machine health verdict and history' ($health -match 'kit machine health (ok|failed) tools=\d+ .* stalled=\d+ history=') (Clip $health)

$marker = Get-Random -Minimum 20000 -Maximum 29999
& $Candidate process start-under-shell cmd /c ping -n $marker 127.0.0.1 *> $null
Start-Sleep -Milliseconds 2000
$ping = Get-CimInstance Win32_Process -Filter "Name='PING.EXE'" | Where-Object { $_.CommandLine -like "*-n $marker*" }
$windows = 0
foreach ($id in @($ping.ProcessId, $ping.ParentProcessId)) {
    foreach ($console in @(Get-CimInstance Win32_Process -Filter "Name='conhost.exe'" | Where-Object { $_.ParentProcessId -eq $id })) {
        $window = Get-Process -Id $console.ProcessId -ErrorAction SilentlyContinue
        if ($window -and $window.MainWindowHandle -ne 0) { $windows++ }
    }
}
if ($ping) { Stop-Process -Id $ping.ProcessId -Force -Confirm:$false -ErrorAction SilentlyContinue; Stop-Process -Id $ping.ParentProcessId -Force -Confirm:$false -ErrorAction SilentlyContinue }
Row 'start-under-shell opens no visible console' ([bool]$ping -and $windows -eq 0) "ping $($ping.ProcessId) visible console windows $windows"

$clock = [Diagnostics.Stopwatch]::StartNew()
$next = (& $Candidate agile next C:\home\git\i574n.github\docs\backlog 50 2>&1 | Select-Object -Last 1 | Out-String)
$clock.Stop()
$ownMs = if ($next -match 'elapsed_ms=(\d+)') { [int]$Matches[1] } else { [int]::MaxValue }
Row 'agile next on the live backlog under 4 s' ($next -match 'kit agile next ok' -and $ownMs -lt 4000) "$ownMs ms by kit's clock, $($clock.ElapsedMilliseconds) ms wall"

''
"acceptance: $(@($rows | Where-Object { $_ -like 'PASS*' }).Count)/$($rows.Count) pass"
