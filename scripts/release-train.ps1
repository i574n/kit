[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Name,
    [string]$KitRoot = (Split-Path $PSScriptRoot -Parent),
    [int]$Throttle = 2,
    [string]$Only = '',
    [switch]$SkipRegen,
    [switch]$NotebooksOnly,
    [switch]$Snapshot,
    [switch]$Resume,
    [string]$Lib = '',
    [string]$Verdicts = ''
)
$ErrorActionPreference = 'Stop'
$KitRoot = (Resolve-Path -LiteralPath $KitRoot).Path
$liveRoot = $KitRoot
$stateRoot = if ($env:KIT_RELEASE_HOME) { $env:KIT_RELEASE_HOME } else { Join-Path $liveRoot '.cache/release-train' }
$out = Join-Path $stateRoot $Name
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'train.log'
function Say([string]$text) { $line = "$(Get-Date -Format 'HH:mm:ss') $text"; Write-Host $line; Add-Content -LiteralPath $log -Value $line }
function Stop-Train([string]$why) { Say "STOP: $why"; exit 1 }
function Get-GeneratedHashes([string]$root) {
    $map = @{}
    Get-ChildItem -LiteralPath (Join-Path $root 'src') -Recurse -File -Filter *.rs | Where-Object { $_.FullName -notmatch '\\target\\' } |
        ForEach-Object { $map[$_.FullName.Substring($root.Length)] = (Get-FileHash -LiteralPath $_.FullName).Hash }
    $map
}
if ($Resume) {
    $snapshotRoot = Join-Path $liveRoot ".cache/release-tree/$Name"
    if (-not (Test-Path -LiteralPath (Join-Path $snapshotRoot 'src'))) { Stop-Train "-Resume needs the snapshot of an earlier -Snapshot run named $Name at $snapshotRoot" }
    if (-not $env:KIT_SPIRAL_BUNDLE) { $env:KIT_SPIRAL_BUNDLE = (Resolve-Path -LiteralPath (Join-Path $liveRoot '../spiral/apps/compiler/tmp')).Path }
    $KitRoot = (Resolve-Path -LiteralPath $snapshotRoot).Path
    $SkipRegen = $true
    Say "resume: reusing snapshot $KitRoot, its candidate and the notebooks that already passed on it"
} elseif ($Snapshot) {
    $snapshotRoot = Join-Path $liveRoot ".cache/release-tree/$Name"
    & robocopy $liveRoot $snapshotRoot /MIR /XD target .cache .git /NFL /NDL /NJH /NJS /NP | Out-Null
    if ($LASTEXITCODE -ge 8) { Stop-Train "snapshot copy failed (robocopy exit $LASTEXITCODE)" }
    $snapshotHashes = Get-GeneratedHashes $snapshotRoot
    if (-not $env:KIT_SPIRAL_BUNDLE) { $env:KIT_SPIRAL_BUNDLE = (Resolve-Path -LiteralPath (Join-Path $liveRoot '../spiral/apps/compiler/tmp')).Path }
    $KitRoot = (Resolve-Path -LiteralPath $snapshotRoot).Path
    Say "snapshot: $liveRoot -> $KitRoot; the live tree stays writable during the train"
}
$src = Join-Path $KitRoot 'src'

$installed = Join-Path $KitRoot 'kit.exe'
Say "train $Name on $KitRoot; installed kit.exe $((Get-FileHash $installed).Hash.Substring(0, 8)); results $out"
. (Join-Path $KitRoot 'compiler-contracts/workspace.ps1')
if (-not $env:SPIRAL_COMPILER_DLL) {
    $liveDll = Join-Path $env:LOCALAPPDATA 'spiral-bin\bin\single-flight\SpiralCompiler\Release\net11.0\SpiralCompiler.dll'
    $pinnedDir = Join-Path $out 'compiler'
    if (-not (Test-Path -LiteralPath (Join-Path $pinnedDir 'SpiralCompiler.dll'))) { [void](Copy-KitCompilerSnapshot -Compiler $liveDll -Destination $pinnedDir) }
    $env:SPIRAL_COMPILER_DLL = Join-Path $pinnedDir 'SpiralCompiler.dll'
}
function Get-CompilerStamp {
    $core = Join-Path (Split-Path $env:SPIRAL_COMPILER_DLL) 'SpiralCompilerCore.dll'
    "host $((Get-FileHash -LiteralPath $env:SPIRAL_COMPILER_DLL).Hash.Substring(0, 16)) core $(if (Test-Path -LiteralPath $core) { (Get-FileHash -LiteralPath $core).Hash.Substring(0, 16) } else { 'none' })"
}
Say "compiler pinned at $env:SPIRAL_COMPILER_DLL ($(Get-CompilerStamp))"

if (-not $SkipRegen) {
    Say 'phase regen: every owner (3 compilers in parallel), cargo check, publish'
    & pwsh -NoProfile -File (Join-Path $KitRoot 'compiler-contracts/test-regeneration.ps1') -KitRoot $KitRoot -IncludeAuxiliary -Incremental -Parallel 3 -FrozenLib $(if ($env:KIT_REGEN_FROZEN_LIB -eq '1' -and $Lib) { Split-Path (Resolve-Path -LiteralPath $Lib).Path } else { '' }) `
        -CargoCheck -Publish -TimeoutSec 1800 *> (Join-Path $out 'regen.log')
    if ($LASTEXITCODE -ne 0) { Get-Content (Join-Path $out 'regen.log') -Tail 15 | ForEach-Object { Say "  regen: $_" }; Stop-Train 'regen failed' }
    Say "phase regen: ok ($((Select-String -Path (Join-Path $out 'regen.log') -Pattern '^Published .*').Line))"
    if ($Snapshot) {
        $copied = 0; $conflicts = 0
        foreach ($entry in (Get-GeneratedHashes $KitRoot).GetEnumerator()) {
            if ($snapshotHashes[$entry.Key] -ceq $entry.Value) { continue }
            $livePath = Join-Path $liveRoot $entry.Key
            $liveHash = if (Test-Path -LiteralPath $livePath) { (Get-FileHash -LiteralPath $livePath).Hash } else { $null }
            if ($liveHash -ceq $snapshotHashes[$entry.Key]) { Copy-Item -LiteralPath (Join-Path $KitRoot $entry.Key) -Destination $livePath -Force; $copied++ }
            else { Say "  regen output not copied back (edited in the live tree since the snapshot): $($entry.Key)"; $conflicts++ }
        }
        Say "snapshot: $copied regenerated outputs copied back to the live tree, $conflicts left for the next in-place regen"
    }
}
& $installed crate gate $src --enforce *> (Join-Path $out 'crate-gate.log')
if ($LASTEXITCODE -ne 0) {
    Get-Content (Join-Path $out 'crate-gate.log') | Where-Object { $_ -match 'OVER|refused' } | ForEach-Object { Say "  gate: $_" }
    Stop-Train 'a generated crate is over the 1000-line gate'
}
Say "phase crate gate: ok ($((Get-Content (Join-Path $out 'crate-gate.log') -Tail 1)))"

Say 'phase candidate: debug build of kit-cli and kit-dev'
$started = Get-Date
Push-Location $src
try { cargo build --locked -p kit-cli -p kit-dev *> (Join-Path $out 'candidate.log') }
finally { Pop-Location }
if ($LASTEXITCODE -ne 0) { Get-Content (Join-Path $out 'candidate.log') -Tail 15 | ForEach-Object { Say "  build: $_" }; Stop-Train 'candidate build failed' }
$candidate = Join-Path $src 'target/debug/kit.exe'
if (-not (Test-Path $candidate)) { Stop-Train "no candidate binary at $candidate" }
$built = (Get-Item $candidate).LastWriteTime
$candidateHash = (Get-FileHash $candidate).Hash.Substring(0, 8)
$earlierCandidate = @(Select-String -LiteralPath $log -Pattern 'phase candidate: ok, kit\.exe (\w{8})' | ForEach-Object { $_.Matches[0].Groups[1].Value }) | Select-Object -Last 1
if ($Resume -and $earlierCandidate -and $earlierCandidate -ne $candidateHash) { Stop-Train "-Resume rebuilt a different candidate ($candidateHash, earlier $earlierCandidate), so the earlier verdicts don't apply; start a fresh -Snapshot train" }
Say "phase candidate: ok, kit.exe $((Get-FileHash $candidate).Hash.Substring(0, 8)) written $($built.ToString('HH:mm:ss'))$(if ($built -lt $started) { ' (cargo found it fresh)' })"

$testBlock = '^(///> |:# (?!.* static =)|test (?!.* static =))'
$binaryUsers = Get-ChildItem $src -Recurse -File -Filter *.spi | Where-Object { $_.FullName -notmatch '[\\/]target[\\/]' } |
    Where-Object { Select-String -LiteralPath $_.FullName -Pattern 'KIT_BIN_UNDER_TEST|kit_run|job_call' -Quiet } |
    Where-Object { Select-String -LiteralPath $_.FullName -Pattern $testBlock -Quiet }
function Get-SourceFiles([string]$dir) {
    foreach ($entry in [IO.Directory]::EnumerateFileSystemEntries($dir)) {
        if ([IO.Directory]::Exists($entry)) { if ([IO.Path]::GetFileName($entry) -notin 'target', '.cache', '.git') { Get-SourceFiles $entry } }
        elseif ([IO.Path]::GetExtension($entry) -in '.spi', '.spir' -or [IO.Path]::GetFileName($entry) -in 'package.spiproj', 'spiral.toml') { $entry }
    }
}
function Get-SourceManifest([string]$root) {
    Get-SourceFiles (Join-Path $root 'src') | ForEach-Object { "$($_.Substring($root.Length + 1).Replace('\', '/')) $((Get-FileHash -LiteralPath $_).Hash)" } | Sort-Object
}
$lastInstalled = Join-Path $stateRoot 'last-installed.txt'
$shippedManifest = if (Test-Path -LiteralPath $lastInstalled) { Join-Path $stateRoot "$((Get-Content -LiteralPath $lastInstalled -TotalCount 1).Trim())/shipped-manifest.txt" } else { '' }
if ($shippedManifest -and (Test-Path -LiteralPath $shippedManifest)) {
    $changedPackages = @(Compare-Object @(Get-Content -LiteralPath $shippedManifest) @(Get-SourceManifest $KitRoot) |
        ForEach-Object { ($_.InputObject -split ' ', 2)[0] } | Where-Object { $_ -match '^src/([^/]+)/' } |
        ForEach-Object { ($_ -split '/')[1] } | Select-Object -Unique)
    Say "changed packages: $($changedPackages.Count), against the last installed train's manifest $shippedManifest"
} else {
    Push-Location $liveRoot
    $changedPackages = @(git status --porcelain -- src | ForEach-Object { $_.Substring(3) } | Where-Object { $_ -match '^src/([^/]+)/' } |
        ForEach-Object { ($_ -split '/')[1] } | Select-Object -Unique)
    Pop-Location
    Say "changed packages: $($changedPackages.Count), from git status (no last-installed manifest yet)"
}
$moduleNotebooks = foreach ($package in $changedPackages) {
    $folder = Join-Path $src $package
    if (Test-Path $folder) {
        Get-ChildItem $folder -File -Filter *.spi | Where-Object { Select-String -LiteralPath $_.FullName -Pattern $testBlock -Quiet }
    }
}
$notebooks = @(@($binaryUsers) + @($moduleNotebooks) | ForEach-Object { $_.FullName.Substring($KitRoot.Length + 1) } | Sort-Object -Unique)
if ($Resume -and (Test-Path -LiteralPath (Join-Path $out 'notebooks.txt'))) { $notebooks = @(Get-Content -LiteralPath (Join-Path $out 'notebooks.txt')) }
if ($Only) { $notebooks = @($notebooks | Where-Object { $_ -match $Only }) }
Set-Content (Join-Path $out 'notebooks.txt') $notebooks
Say "notebook set: $($notebooks.Count) ($(@($binaryUsers).Count) use the binary; module notebooks of $($changedPackages.Count) changed packages)"

if ($Verdicts) {
    $given = @{}
    Get-Content -LiteralPath $Verdicts | Where-Object { $_ -match '^pass\s+(\S+)' } | ForEach-Object { $given[$Matches[1]] = $_ }
    $missing = @($notebooks | Where-Object { -not $given.ContainsKey($_) })
    if ($missing.Count) { Stop-Train "verdicts in $Verdicts miss or fail $($missing.Count) of $($notebooks.Count) notebooks: $($missing -join ', ')" }
    Copy-Item -LiteralPath $Verdicts -Destination (Join-Path $out 'notebooks-verdicts.txt') -Force
    Say "phase notebooks: accepted $($notebooks.Count)/$($notebooks.Count) pass verdicts from $Verdicts (the same tree, run on another machine)"
} else {
$spiral = (Resolve-Path -LiteralPath (Join-Path $liveRoot '../spiral')).Path
$runner = Join-Path $spiral 'apps/cells/spi/run_notebook.ps1'
$spiralLib = if ($Lib) { (Resolve-Path -LiteralPath $Lib).Path } else { Join-Path $spiral 'lib/spiral' }
if ($Lib -and $KitRoot -ne $liveRoot) {
    $nearest = Join-Path $KitRoot 'deps/spiral/lib/spiral'
    & robocopy $spiralLib $nearest /MIR /NFL /NDL /NJH /NJS /NP | Out-Null
    if ($LASTEXITCODE -ge 8 -or -not ((Test-Path -LiteralPath (Join-Path $nearest 'package.spiproj')) -or (Test-Path -LiteralPath (Join-Path $nearest 'spiral.toml')))) { Stop-Train "lib snapshot copy into $nearest failed (robocopy exit $LASTEXITCODE)" }
    $spiralLib = $nearest
}
Say "notebooks read the lib at $spiralLib"
$env:SPIRAL_LIB = $spiralLib
$env:SPIRAL_CELLS_LIB = $spiralLib
$env:KIT_BIN_UNDER_TEST = $candidate
$env:SPIRAL_CELLS_WARM = '0'; $env:SPIRAL_CELLS_MEMO = '0'; $env:SPIRAL_CELLS_COMPILER = 'process'
$env:MIX_BUILD_PATH = Join-Path $out 'mix-control'
$control = 'src/kit_lcov/lcov.spi'
Say 'phase notebooks: start'
Push-Location $KitRoot
& pwsh -NoProfile -File $runner --path (Join-Path $KitRoot $control) --output-path (Join-Path $out 'positive-control.ipynb') --no-html *> (Join-Path $out 'positive-control.log')
$controlExit = $LASTEXITCODE
Pop-Location
if ($controlExit -ne 0) { Get-Content (Join-Path $out 'positive-control.log') -Tail 8 | ForEach-Object { Say "  control: $_" }; Stop-Train "positive control $control failed, so notebook failures would be the environment, not the candidate" }
Say "positive control ${control}: pass"

$runOne = {
    $notebook = $_
    $name = $notebook -replace '[\\/]', '_'
    $notebookLog = Join-Path $using:out "$name.log"
    $env:KIT_BIN_UNDER_TEST = $using:candidate
    $env:SPIRAL_LIB = $using:spiralLib
    $env:SPIRAL_CELLS_LIB = $using:spiralLib
    $env:SPIRAL_CELLS_WARM = '0'; $env:SPIRAL_CELLS_MEMO = '0'; $env:SPIRAL_CELLS_COMPILER = 'process'
    $slot = -1; $slotMutex = $null
    for ($index = 0; $index -lt $using:Throttle -and $slot -lt 0; $index++) {
        $candidateMutex = [System.Threading.Mutex]::new($false, "Local\kit-train-$($using:Name)-slot-$index")
        $owned = try { $candidateMutex.WaitOne(0) } catch [System.Threading.AbandonedMutexException] { $true }
        if ($owned) { $slot = $index; $slotMutex = $candidateMutex } else { $candidateMutex.Dispose() }
    }
    $env:MIX_BUILD_PATH = Join-Path $using:out "mix-worker-$slot"
    Set-Location $using:KitRoot
    $runnerArgs = @('-NoProfile', '-File', $using:runner, '--path', (Join-Path $using:KitRoot $notebook), '--output-path', (Join-Path $using:out "$name.ipynb"), '--no-html')
    $childEnvironment = @{
        MIX_BUILD_PATH = Join-Path $using:out "mix-worker-$slot"
        KIT_BIN_UNDER_TEST = $using:candidate
        SPIRAL_LIB = $using:spiralLib
        SPIRAL_CELLS_LIB = $using:spiralLib
        SPIRAL_CELLS_WARM = '0'
        SPIRAL_CELLS_MEMO = '0'
        SPIRAL_CELLS_COMPILER = 'process'
    }
    $proc = Start-Process pwsh -ArgumentList $runnerArgs -Environment $childEnvironment -RedirectStandardOutput $notebookLog -RedirectStandardError "$notebookLog.err" -PassThru -NoNewWindow
    $holders = @{}
    while (-not $proc.HasExited) {
        $seen = "$(Get-Content -Raw -LiteralPath $notebookLog -ErrorAction SilentlyContinue)"
        foreach ($match in [regex]::Matches($seen, 'held by process (\d+)')) {
            $holder = $match.Groups[1].Value
            if (-not $holders.ContainsKey($holder)) {
                $found = Get-CimInstance Win32_Process -Filter "ProcessId=$holder" -ErrorAction SilentlyContinue
                $holders[$holder] = if ($found) { "$($found.Name) parent=$($found.ParentProcessId) $($found.CommandLine)" } else { 'gone before capture' }
            }
        }
        Start-Sleep -Milliseconds 500
    }
    $proc.WaitForExit()
    Add-Content -LiteralPath $notebookLog -Value "$(Get-Content -Raw -LiteralPath "$notebookLog.err" -ErrorAction SilentlyContinue)"
    foreach ($entry in $holders.GetEnumerator()) { Add-Content -LiteralPath $notebookLog -Value "lock holder $($entry.Key): $($entry.Value)" }
    $code = $proc.ExitCode
    if ($slotMutex) { $slotMutex.ReleaseMutex(); $slotMutex.Dispose() }
    $text = Get-Content -Raw -LiteralPath $notebookLog
    $failed = [regex]::Match($text, '"failed_tests":\[[^\]]*\]').Value
    $total = [regex]::Match($text, 'spiral-tests total (\{[^\r\n]*\})')
    $totals = if ($total.Success) { try { $total.Groups[1].Value | ConvertFrom-Json } catch { $null } } else { $null }
    $ran = $totals -and $totals.unsupported -eq 0 -and ($totals.executed -gt 0 -or $totals.discovered -eq 0)
    $verdict = if ($text -match 'ETY-[A-Za-z]+|ParserErrors|type error somewhere') { 'TYPE-ERROR' }
        elseif (-not $ran) { 'NOT-RUN' }
        elseif ($code -eq 0 -and $totals.passed -eq $totals.executed) { 'pass' }
        else { 'runtime-fail' }
    '{0,-12} {1} exit={2} {3}' -f $verdict, $notebook, $code, $failed
}
$verdictsFile = Join-Path $out 'notebooks-verdicts.txt'
$passedBefore = @{}
if ($Resume -and (Test-Path -LiteralPath $verdictsFile)) {
    Get-Content -LiteralPath $verdictsFile | Where-Object { $_ -match '^pass\s+(\S+)' -and $Matches[1] -in $notebooks } | ForEach-Object { $passedBefore[$Matches[1]] = $_ }
    Say "resume: $($passedBefore.Count) notebooks already passed on this candidate; running the other $($notebooks.Count - $passedBefore.Count)"
} elseif ($Resume) {
    foreach ($notebook in $notebooks) {
        $notebookLog = Join-Path $out "$($notebook -replace '[\\/]', '_').log"
        if (-not (Test-Path -LiteralPath $notebookLog)) { continue }
        $logged = (Get-Item -LiteralPath $notebookLog).LastWriteTime
        $source = Join-Path (Split-Path $src -Parent) $notebook
        if ($logged -lt $built -or ((Test-Path -LiteralPath $source) -and $logged -lt (Get-Item -LiteralPath $source).LastWriteTime)) { continue }
        $text = Get-Content -Raw -LiteralPath $notebookLog
        $total = [regex]::Match("$text", 'spiral-tests total (\{[^\r\n]*\})')
        $totals = if ($total.Success) { try { $total.Groups[1].Value | ConvertFrom-Json } catch { $null } } else { $null }
        $clean = $totals -and $totals.unsupported -eq 0 -and $totals.failed -eq 0 -and $totals.passed -eq $totals.executed -and ($totals.executed -gt 0 -or $totals.discovered -eq 0)
        if ($clean -and $text -notmatch 'ETY-[A-Za-z]+|ParserErrors|type error somewhere|timed out') { $passedBefore[$notebook] = '{0,-12} {1} exit=0 (from the earlier run''s log)' -f 'pass', $notebook }
    }
    Say "resume: the earlier run wrote no verdicts (it was stopped mid-phase); $($passedBefore.Count) notebooks passed by their logs; running the other $($notebooks.Count - $passedBefore.Count)"
}
$pending = @($notebooks | Where-Object { -not $passedBefore.ContainsKey($_) })
$rows = @($passedBefore.Values) + @($pending | ForEach-Object -ThrottleLimit $Throttle -Parallel $runOne)
$timedOut = @($rows | Where-Object { $_ -notmatch '^pass ' } | ForEach-Object { ($_ -split '\s+')[1] } |
    Where-Object { Select-String -LiteralPath (Join-Path $out "$($_ -replace '[\\/]', '_').log") -Pattern 'Spiral cell timed out' -Quiet -ErrorAction SilentlyContinue })
if ($timedOut.Count) {
    Say "phase notebooks: $($timedOut.Count) timed out under load (a starved run is void, not a failure); rerunning them one at a time"
    $retried = @($timedOut | ForEach-Object -ThrottleLimit 1 -Parallel $runOne)
    $rows = @($rows | Where-Object { ($_ -split '\s+')[1] -notin $timedOut }) + $retried
}
$rows = @($rows | Sort-Object { ($_ -split '\s+')[1] })
Set-Content (Join-Path $out 'notebooks-verdicts.txt') $rows
$bad = @($rows | Where-Object { $_ -notmatch '^pass ' })
Say "phase notebooks: end, $($rows.Count - $bad.Count)/$($rows.Count) pass (notebooks-verdicts.txt); compiler at end $(Get-CompilerStamp)"
$bad | ForEach-Object { Say "  $_" }
if ($NotebooksOnly) { Say 'DONE (notebooks only, nothing installed)'; exit $(if ($bad.Count) { 1 } else { 0 }) }
if ($bad.Count) { Stop-Train "$($bad.Count) candidate notebooks are not green; nothing installed" }
}

Say 'phase release: build.ps1 (release build, schema and topology checks, publish kit.exe)'
Say "phase release: compiler $env:SPIRAL_COMPILER_DLL ($(Get-CompilerStamp))"
& pwsh -NoProfile -File (Join-Path $KitRoot 'build.ps1') -TimeoutMs 3600000 -SpiralCompiler $env:SPIRAL_COMPILER_DLL *> (Join-Path $out 'release.log')
if ($LASTEXITCODE -ne 0) { Get-Content (Join-Path $out 'release.log') -Tail 15 | ForEach-Object { Say "  release: $_" }; Stop-Train 'release build failed; installed kit.exe unchanged' }
Say "phase release: installed kit.exe $((Get-FileHash $installed).Hash.Substring(0, 8)) ($((Select-String -Path (Join-Path $out 'release.log') -Pattern 'published=').Line))"

Say 'phase smoke: the installed binary answers read verbs'
$smoke = @(& $installed job list 3 2>&1)
if ($LASTEXITCODE -ne 0) { $smoke | ForEach-Object { Say "  smoke: $_" }; Stop-Train 'installed kit.exe fails job list' }
if ($Snapshot -or $Resume) {
    $liveBinary = Join-Path $liveRoot 'kit.exe'
    $liveStaged = "$liveBinary.next"
    Copy-Item -LiteralPath $installed -Destination $liveStaged -Force
    $retiredRepo = Join-Path $env:LOCALAPPDATA 'kit\retired'
    [void][IO.Directory]::CreateDirectory($retiredRepo)
    if (Test-Path -LiteralPath $liveBinary) { [IO.File]::Move($liveBinary, (Join-Path $retiredRepo "repo-kit-$(Get-Date -Format yyyyMMddHHmmssfff).exe")) }
    [IO.File]::Move($liveStaged, $liveBinary)
    Say "snapshot: live repo kit.exe refreshed: $((Get-FileHash $liveBinary).Hash.Substring(0, 8))"
}
$laneBin = Join-Path $env:LOCALAPPDATA 'kit\bin'
if ($env:LOCALAPPDATA) {
    [void][IO.Directory]::CreateDirectory($laneBin)
    $laneCopy = Join-Path $laneBin 'kit.exe'
    $staged = "$laneCopy.next"
    Copy-Item -LiteralPath $installed -Destination $staged -Force
    $retiredRoot = Join-Path $env:LOCALAPPDATA 'kit\retired'
    [void][IO.Directory]::CreateDirectory($retiredRoot)
    try {
        if (Test-Path -LiteralPath $laneCopy) { [IO.File]::Move($laneCopy, (Join-Path $retiredRoot "lane-kit-$(Get-Date -Format yyyyMMddHHmmssfff).exe")) }
        [IO.File]::Move($staged, $laneCopy)
        Say "lane copy $laneCopy refreshed: $((Get-FileHash $laneCopy).Hash.Substring(0, 8)) (the previous build was renamed into $retiredRoot, so running lanes kept working)"
    }
    catch { Say "lane copy swap failed ($($_.Exception.Message)); the new one waits as $staged" }
}
Get-SourceManifest $KitRoot | Set-Content -LiteralPath (Join-Path $out 'shipped-manifest.txt')
Set-Content -LiteralPath $lastInstalled $Name
Say "shipped manifest written; the next train tests every package changed since $Name, committed or not"
Say 'DONE'
