[CmdletBinding()]
param(
    [string]$KitRoot,
    [string]$Filter = '.*',
    [string[]]$Package = @(),
    [switch]$IncludeAuxiliary,
    [string]$TargetDirectory,
    [string]$CompilerBundle = $env:KIT_SPIRAL_BUNDLE,
    [ValidateRange(1, 1800)][int]$TimeoutSec = 180,
    [switch]$CargoCheck,
    [switch]$Test,
    [switch]$SkipRelease,
    [switch]$Publish,
    [switch]$CompilerContracts,
    [switch]$Offline,
    [switch]$KeepStaged,
    [switch]$Incremental,
    [ValidateRange(1, 8)][int]$Parallel = 1,
    [string]$FrozenLib = ''
)
$ErrorActionPreference = 'Stop'
if ($TargetDirectory) { $TargetDirectory = [IO.Path]::GetFullPath($TargetDirectory, $PWD.Path) }
if ($CompilerContracts -and -not $Test) { throw '-CompilerContracts requires -Test.' }
if ($Publish -and -not ($CargoCheck -or $Test)) { throw '-Publish requires -CargoCheck or -Test.' }
. $PSScriptRoot/workspace.ps1
$publishLeases = @()
trap { foreach ($lease in $publishLeases) { foreach ($try in 1..10) { Remove-Item -LiteralPath $lease -ErrorAction SilentlyContinue; if (-not (Test-Path -LiteralPath $lease)) { break }; Start-Sleep -Milliseconds 200 } }; break }
if (-not $KitRoot) { $KitRoot = Join-Path $PSScriptRoot '..' }
$KitRoot = (Resolve-Path -LiteralPath $KitRoot).Path
$selected = @(Get-KitOwners -Root $KitRoot -Package $Package -Filter $Filter -IncludeAuxiliary:$IncludeAuxiliary)
if (-not $selected.Count) { throw 'No regeneration owners selected.' }
function Get-ProjectSection([string]$Member, [string[]]$Sections) {
    $project = Join-Path $KitRoot "src/$Member/package.spiproj"
    if (Test-Path -LiteralPath $project) {
        $section = ''
        foreach ($line in Get-Content -LiteralPath $project) {
            if ($line -match '^(\S[^:]*):') { $section = $Matches[1]; continue }
            if ($section -in $Sections -and $line -match '^\s+\|?([A-Za-z0-9_]+)\s*$') { $Matches[1] }
        }
    }
}
$linkedOwners = @{}
$linkQueue = [Collections.Generic.Queue[string]]::new()
foreach ($owner in $selected) { $linkQueue.Enqueue($owner.Member) }
while ($linkQueue.Count) {
    foreach ($link in Get-ProjectSection $linkQueue.Dequeue() @('link')) {
        if ($link -in $selected.Member -or $linkedOwners.ContainsKey($link)) { continue }
        $linkedOwners[$link] = @(Get-KitOwners -Root $KitRoot -Package $link -IncludeAuxiliary)[0]
        $linkQueue.Enqueue($link)
    }
}
$byMember = @{}
foreach ($owner in @($selected) + @($linkedOwners.Values)) { $byMember[$owner.Member] = $owner }
$ordered = [Collections.Generic.List[object]]::new()
$placed = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
function Add-LinkedFirst([string]$Member, [string[]]$Trail) {
    if ($placed.Contains($Member)) { return }
    if ($Member -in $Trail) { throw "link: cycle: $(($Trail + $Member) -join ' -> ')" }
    foreach ($link in Get-ProjectSection $Member @('link')) { if ($byMember.ContainsKey($link)) { Add-LinkedFirst $link ($Trail + $Member) } }
    if ($placed.Add($Member)) { $ordered.Add($byMember[$Member]) }
}
foreach ($owner in $selected) { Add-LinkedFirst $owner.Member @() }
if ($linkedOwners.Count) { Write-Host "link: units compiled first: $(($linkedOwners.Keys | Sort-Object) -join ', ')" }
$selected = @($ordered)
if ($Package.Count) { $Package = @($selected | ForEach-Object Package | Select-Object -Unique) }
function Get-PackageInputs([string]$Member) {
    $folder = Join-Path $KitRoot "src/$Member"
    Get-ProjectSection $Member @('packages', 'link')
    $manifest = Join-Path $folder 'Cargo.toml'
    if (Test-Path -LiteralPath $manifest) {
        foreach ($match in [regex]::Matches([IO.File]::ReadAllText($manifest), 'path\s*=\s*"\.\./([^"/]+)"')) { $match.Groups[1].Value }
    }
}
$watchedMembers = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
$pendingMembers = [Collections.Generic.Queue[string]]::new()
foreach ($owner in $selected) { if ($watchedMembers.Add($owner.Member)) { $pendingMembers.Enqueue($owner.Member) } }
while ($pendingMembers.Count) {
    foreach ($dependency in Get-PackageInputs $pendingMembers.Dequeue()) {
        if ((Test-Path -LiteralPath (Join-Path $KitRoot "src/$dependency") -PathType Container) -and $watchedMembers.Add($dependency)) { $pendingMembers.Enqueue($dependency) }
    }
}
$watchedDirectories = @(@($watchedMembers | ForEach-Object { [IO.Path]::GetFullPath((Join-Path $KitRoot "src/$_")) }) + [IO.Path]::GetFullPath((Join-Path $KitRoot 'compiler-contracts')))
$watchedFiles = @(@(Get-ChildItem -LiteralPath $KitRoot -File) + @(Get-ChildItem -LiteralPath (Join-Path $KitRoot 'src') -File) | Where-Object { $_.Name -notin @('kit', 'kit.exe') } | ForEach-Object FullName)
function Test-Watched([string]$Path) {
    if ($Path -in $watchedFiles) { return $true }
    foreach ($directory in $watchedDirectories) { if ($Path.StartsWith($directory + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { return $true } }
    $false
}
if ($Publish) {
    $leaseDir = Join-Path ($env:LOCALAPPDATA ?? $HOME) 'kit/tree-leases'
    New-Item -ItemType Directory -Force $leaseDir | Out-Null
    $lease = Join-Path $leaseDir "regeneration-$PID.lease"
    $rootLines = @(@($watchedDirectories) + @($watchedFiles) | ForEach-Object { "root=$_" })
    (@($rootLines) + "pid=$PID" + "holder=kit regeneration -Publish of $($selected.Count) owners" + "since=$(Get-Date -Format HH:mm)") | Set-Content $lease
    $publishLeases += $lease
    Write-Host "Leased $($watchedDirectories.Count) package folders and $($watchedFiles.Count) workspace files; the rest of $KitRoot stays writable."
}
$CompilerBundle = Resolve-KitSpiralBundle $CompilerBundle
. (Get-KitSpiralScript $CompilerBundle 'env.ps1')
$dotnet = Resolve-SpiralDotnet
$compiler = Get-SpiralCompilerDll 'single-flight'
$regenerationRoot = Join-Path (Get-SpiralCacheDir) 'kit-regeneration'
foreach ($stale in @(Get-ChildItem -LiteralPath $regenerationRoot -Directory -ErrorAction SilentlyContinue | Where-Object LastWriteTime -lt (Get-Date).AddDays(-2))) {
    try { Remove-Item -LiteralPath $stale.FullName -Recurse -Force -ErrorAction Stop } catch { Write-Host "Could not prune stale regeneration workspace $($stale.FullName): $($_.Exception.Message)" }
}
$work = Join-Path $regenerationRoot ([guid]::NewGuid().ToString('N'))
$compiler = Copy-KitCompilerSnapshot -Compiler $compiler -Destination (Join-Path $work 'compiler')
$staged = Join-Path $work 'kit'
$snapshot = [Collections.Generic.List[object]]::new()
function Get-WorkspaceEntries([string]$Directory) {
    foreach ($entry in Get-ChildItem -LiteralPath $Directory -Force) {
        if ($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Unexpected workspace link: $($entry.FullName)" }
        if ($entry.PSIsContainer) {
            if ($entry.Name -notin @('target', 'vendor', '.git', '.cache')) { $entry }
        } elseif ($entry.Name -notin @('kit', 'kit.exe')) { $entry }
    }
}
function Get-WorkspaceFiles([string]$Directory) {
    foreach ($entry in Get-WorkspaceEntries $Directory) {
        if ($entry.PSIsContainer) { Get-WorkspaceFiles $entry.FullName }
        else { $entry.FullName }
    }
}
function Copy-Workspace([string]$From, [string]$To) {
    New-Item -ItemType Directory -Path $To -Force | Out-Null
    foreach ($entry in Get-WorkspaceEntries $From) {
        if ($entry.PSIsContainer) {
            Copy-Workspace $entry.FullName (Join-Path $To $entry.Name)
        } else {
            $copy = Join-Path $To $entry.Name
            Copy-Item -LiteralPath $entry.FullName -Destination $copy
            if ($Publish -and (Test-Watched $entry.FullName)) {
                $snapshot.Add([pscustomobject]@{ Path = $entry.FullName; Hash = (Get-FileHash -LiteralPath $copy -Algorithm SHA256).Hash })
            }
        }
    }
}
Copy-Workspace $KitRoot $staged
$spiralLib = Join-Path $CompilerBundle '../../../lib/spiral'
$libReaders = @(Get-ChildItem -LiteralPath (Join-Path $staged 'src') -Directory | Where-Object { $project = Join-Path $_.FullName 'package.spiproj'; [IO.File]::Exists($project) -and [IO.File]::ReadAllText($project) -match '(?m)^\s*\|?spiral-\s*$' })
$libPresent = (Test-Path -LiteralPath (Join-Path $spiralLib 'package.spiproj')) -or (Test-Path -LiteralPath (Join-Path $spiralLib 'spiral.toml'))
if ($libReaders.Count -and -not $libPresent) { throw "Spiral lib not found at $spiralLib; $($libReaders[0].Name) and $($libReaders.Count - 1) other packages reference it as spiral-." }
$stagedLib = Join-Path $staged 'src/spiral'
$libRoot = if ($libPresent) { (Resolve-Path -LiteralPath $spiralLib).Path } else { '' }
if ($FrozenLib) {
    if (-not (Test-Path -LiteralPath (Join-Path $FrozenLib 'spiral'))) { throw "-FrozenLib needs a folder holding spiral/ (got $FrozenLib)" }
    $env:SPIRAL_LIB_ROOT = (Resolve-Path -LiteralPath $FrozenLib).Path
    $libRoot = (Resolve-Path -LiteralPath (Join-Path $FrozenLib 'spiral')).Path
    Write-Host "Compiling against the frozen lib $libRoot (SPIRAL_LIB_ROOT)"
}
foreach ($file in $(if ($libRoot) { Get-ChildItem -LiteralPath $libRoot -Recurse -File -Include *.spi, *.spir, package.spiproj, spiral.toml })) {
    $relative = $file.FullName.Substring($libRoot.Length).TrimStart('\', '/')
    if ($relative -match '(^|[\\/])(target|node_modules|bench)[\\/]') { continue }
    $destination = Join-Path $stagedLib $relative
    New-Item -ItemType Directory -Path (Split-Path $destination) -Force | Out-Null
    Copy-Item -LiteralPath $file.FullName -Destination $destination -Force
}
$sourceRoot = Join-Path $staged 'src'
if (-not $TargetDirectory) { $TargetDirectory = Join-Path $sourceRoot 'target' }
$rows = @()
foreach ($owner in $selected) {
    $entry = Join-Path $staged ([IO.Path]::GetRelativePath($KitRoot, $owner.Input))
    $target = Join-Path $staged ([IO.Path]::GetRelativePath($KitRoot, $owner.Target))
    $output = Join-Path $work "outputs/$($owner.Id).rs"
    $rows += [pscustomobject]@{ Member = $owner.Id; Package = $owner.Package; Input = $entry; Output = $output; Target = $target; Original = $owner.Target }
}
$keyStore = Join-Path ($env:LOCALAPPDATA ?? $HOME) 'kit/regen-keys'
$ownerKeys = @{}
if ($Incremental) {
    New-Item -ItemType Directory -Force $keyStore | Out-Null
    $shared = [Text.StringBuilder]::new()
    [void]$shared.AppendLine("compiler $((Get-FileHash -LiteralPath $compiler -Algorithm SHA256).Hash)")
    foreach ($file in $($(if ($libRoot) { Get-ChildItem -LiteralPath $libRoot -Recurse -File -Include *.spi, *.spir, package.spiproj, spiral.toml }) | Where-Object { $_.FullName -notmatch '[\\/](target|node_modules|bench)[\\/]' } | Sort-Object FullName)) {
        [void]$shared.AppendLine("lib $($file.FullName.Substring($libRoot.Length)) $((Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash)")
    }
    $sharedText = $shared.ToString()
    foreach ($owner in $selected) {
        $closure = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
        $queue = [Collections.Generic.Queue[string]]::new()
        if ($closure.Add($owner.Member)) { $queue.Enqueue($owner.Member) }
        while ($queue.Count) {
            foreach ($dependency in Get-PackageInputs $queue.Dequeue()) {
                if ((Test-Path -LiteralPath (Join-Path $KitRoot "src/$dependency") -PathType Container) -and $closure.Add($dependency)) { $queue.Enqueue($dependency) }
            }
        }
        $lines = [Text.StringBuilder]::new($sharedText)
        foreach ($member in @($closure | Sort-Object)) {
            foreach ($file in @(Get-ChildItem -LiteralPath (Join-Path $KitRoot "src/$member") -File -Include *.spi, *.spir, package.spiproj, Cargo.toml -Recurse | Where-Object { $_.FullName -notmatch '[\\/]target[\\/]' } | Sort-Object FullName)) {
                [void]$lines.AppendLine("src $($file.FullName.Substring($KitRoot.Length)) $((Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash)")
            }
        }
        $ownerKeys[$owner.Id] = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($lines.ToString())))
    }
}
function Get-StoredKey([string]$Id) {
    $file = Join-Path $keyStore "$($Id -replace '[\\/]', '__').key"
    if (Test-Path -LiteralPath $file) { (Get-Content -LiteralPath $file -Raw).Trim() -split ' ' } else { @('', '') }
}
$sidecars = @(Get-KitCompilerSidecarGuards -Rows $rows)
New-Item -ItemType Directory -Path (Join-Path $work 'outputs') -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $work 'logs') -Force | Out-Null
$results = Join-Path $work 'results.tsv'
$rows | Export-Csv -LiteralPath (Join-Path $work 'owners.csv') -NoTypeInformation
$previousWorkspace = $env:SPIRAL_WORKSPACE_ROOT
$previousBudget = $env:SPIRAL_BUILD_BUDGET_MS
try {
    $env:SPIRAL_WORKSPACE_ROOT = $BundleRoot
    Write-Host "Regenerating $($rows.Count) declared KIT outputs: $work"
    $encoding = [Text.UTF8Encoding]::new($false)
    $writer = [IO.StreamWriter]::new($results, $false, $encoding)
    try {
        $writer.AutoFlush = $true
        $timeoutMs = $TimeoutSec * 1000
        $env:SPIRAL_BUILD_BUDGET_MS = [string]([Math]::Max(1000, $timeoutMs - 3000))
        $linksOf = @{}
        foreach ($row in $rows) { $linksOf[$row.Member] = @(Get-ProjectSection $row.Member @('link')) }
        $pending = [Collections.Generic.List[object]]::new()
        foreach ($row in $rows) { $pending.Add($row) }
        $running = [Collections.Generic.List[object]]::new()
        function Complete-Owner($job) {
            $row = $job.Row
            if ($job.TimedOut) {
                $writer.WriteLine("$($row.Member)`ttimeout`t$($job.Watch.ElapsedMilliseconds)`tno result within $timeoutMs ms")
                Write-Host "$($row.Member) timeout $($job.Watch.ElapsedMilliseconds) ms"
                return
            }
            $elapsed = $job.Watch.ElapsedMilliseconds
            if ($job.ExitCode -eq 0 -and (Test-Path -LiteralPath $row.Output)) {
                $bytes = (Get-Item -LiteralPath $row.Output).Length
                $writer.WriteLine("$($row.Member)`tok`t$elapsed`tbytes=$bytes entry=main revision=process")
                Write-Host "$($row.Member) ok $elapsed ms ($bytes bytes)"
            } else {
                $detail = ''
                if (Test-Path -LiteralPath $job.Stderr) { $detail = ([IO.File]::ReadAllText($job.Stderr) -replace '[\t\r\n]', ' ').Trim() }
                if (-not $detail -and (Test-Path -LiteralPath $job.Stdout)) { $detail = ([IO.File]::ReadAllText($job.Stdout) -replace '[\t\r\n]', ' ').Trim() }
                if ($detail.Length -gt 500) { $detail = $detail.Substring(0, 500) }
                if (-not $detail) { $detail = "compiler exit $($job.ExitCode)" }
                $writer.WriteLine("$($row.Member)`terror`t$elapsed`t$detail")
                Write-Host "$($row.Member) error $elapsed ms $detail"
            }
        }
        while ($pending.Count -or $running.Count) {
            $busy = @($running | ForEach-Object { $_.Row.Member })
            $next = $null
            if ($running.Count -lt $Parallel) {
                foreach ($candidate in $pending) {
                    $waits = @($linksOf[$candidate.Member] | Where-Object { $_ -in $busy -or $_ -in @($pending | ForEach-Object Member) })
                    if (-not $waits.Count) { $next = $candidate; break }
                }
            }
            if ($next) {
                [void]$pending.Remove($next)
                $row = $next
                if ($Incremental) {
                    $stored = Get-StoredKey $row.Member
                    if ($stored[0] -and $stored[0] -eq $ownerKeys[$row.Member] -and (Test-Path -LiteralPath $row.Original) -and (Get-FileHash -LiteralPath $row.Original -Algorithm SHA256).Hash -eq $stored[1]) {
                        Copy-Item -LiteralPath $row.Original -Destination $row.Output -Force
                        $writer.WriteLine("$($row.Member)`tok`t0`tbytes=$((Get-Item -LiteralPath $row.Output).Length) entry=main revision=unchanged-inputs")
                        Write-Host "$($row.Member) unchanged inputs; compile skipped"
                        continue
                    }
                }
                $start = [Diagnostics.ProcessStartInfo]::new()
                $start.FileName = $dotnet
                $start.WorkingDirectory = $work
                $start.UseShellExecute = $false
                $start.CreateNoWindow = $true
                $start.RedirectStandardOutput = $true
                $start.RedirectStandardError = $true
                foreach ($argument in @($compiler, '--backend', 'Rust', $row.Input, $row.Output)) { $start.ArgumentList.Add($argument) }
                $proc = [Diagnostics.Process]::new()
                $proc.StartInfo = $start
                if (-not $proc.Start()) { throw "Cannot start compiler for $($row.Member)" }
                $running.Add([pscustomobject]@{
                    Row = $row; Proc = $proc; Watch = [Diagnostics.Stopwatch]::StartNew()
                    OutTask = $proc.StandardOutput.ReadToEndAsync(); ErrTask = $proc.StandardError.ReadToEndAsync()
                    Stdout = (Join-Path $work "logs/$($row.Member).out"); Stderr = (Join-Path $work "logs/$($row.Member).err")
                    TimedOut = $false; ExitCode = -1
                })
                continue
            }
            foreach ($job in @($running)) {
                $done = $job.Proc.HasExited
                if (-not $done -and $job.Watch.ElapsedMilliseconds -ge $timeoutMs) {
                    $job.Proc.Kill($true)
                    if (-not $job.Proc.WaitForExit(10000)) { throw "Compiler did not terminate: $($job.Row.Member)" }
                    $job.TimedOut = $true
                    $done = $true
                }
                if (-not $done) { continue }
                $job.Proc.WaitForExit()
                [IO.File]::WriteAllText($job.Stdout, $job.OutTask.GetAwaiter().GetResult())
                [IO.File]::WriteAllText($job.Stderr, $job.ErrTask.GetAwaiter().GetResult())
                if (-not $job.TimedOut) { $job.ExitCode = $job.Proc.ExitCode }
                $job.Proc.Dispose()
                [void]$running.Remove($job)
                Complete-Owner $job
            }
            if ($running.Count -ge $Parallel -or (-not $next -and $running.Count)) { Start-Sleep -Milliseconds 100 }
        }
    } finally {
        $writer.Dispose()
    }
} finally {
    $env:SPIRAL_WORKSPACE_ROOT = $previousWorkspace
    if ($null -eq $previousBudget) { Remove-Item Env:SPIRAL_BUILD_BUDGET_MS -ErrorAction SilentlyContinue }
    else { $env:SPIRAL_BUILD_BUDGET_MS = $previousBudget }
}
$compiled = @(Import-Csv -LiteralPath $results -Delimiter "`t" -Header 'owner','status','elapsed','detail')
$failed = @($compiled | Where-Object status -ne 'ok')
Write-Host "Emitted: $(@($compiled | Where-Object status -eq 'ok').Count)/$($rows.Count); results: $results"
if ($failed.Count -or $compiled.Count -ne $rows.Count) {
    $failed | Format-Table owner,status,detail -Wrap
    throw "KIT regeneration incomplete; committed Rust was not replaced. Results: $results"
}
Restore-KitCompilerSidecars -Root $staged -Guards $sidecars
foreach ($row in $rows) {
    if (-not (Test-Path -LiteralPath $row.Output)) { throw "Missing emitted owner: $($row.Member)" }
    Copy-Item -LiteralPath $row.Output -Destination $row.Target
}
if ($CargoCheck) {
    $cargoArgs = @('check', '--target-dir', $TargetDirectory, '--all-targets', '--locked')
    if ($Package.Count) {
        foreach ($name in $Package) { $cargoArgs += @('--package', $name) }
    } else { $cargoArgs += '--workspace' }
    if ($Offline) { $cargoArgs += '--offline' }
    Push-Location $sourceRoot
    try {
        $cargoLines = @(& cargo @cargoArgs 2>&1 | ForEach-Object { "$_" })
        $cargoLines | ForEach-Object { Write-Host $_ }
        if ($LASTEXITCODE -ne 0) {
            $firstError = @($cargoLines | Select-String -Pattern '^error' -Context 0, 10 | Select-Object -First 1 | ForEach-Object { @($_.Line) + $_.Context.PostContext }) -join "`n"
            throw "Cargo rejected regenerated KIT owners: $staged`n$firstError"
        }
    } finally { Pop-Location }
}
if ($Test) {
    $previousDotnet = $env:KIT_DOTNET
    $previousBundle = $env:KIT_SPIRAL_BUNDLE
    try {
        $env:KIT_DOTNET = $dotnet
        $env:KIT_SPIRAL_BUNDLE = $BundleRoot
        & (Join-Path $staged 'build.ps1') -Test -Package $Package -TargetDirectory $TargetDirectory -SkipRelease:$SkipRelease -CompilerContracts:$CompilerContracts -Offline:$Offline -SpiralCompiler $compiler
        if ($LASTEXITCODE -ne 0) { throw "Regenerated KIT runtime contracts failed: $staged" }
    } finally {
        $env:KIT_DOTNET = $previousDotnet
        $env:KIT_SPIRAL_BUNDLE = $previousBundle
    }
    if (-not $Package.Count) {
        $binaryName = if ($IsWindows) { 'kit.exe' } else { 'kit' }
        $validatedBinary = Join-Path $TargetDirectory "debug/$binaryName"
        & $validatedBinary proxy source-topology $staged
        if ($LASTEXITCODE -ne 0) { throw "Regenerated KIT source topology failed: $staged" }
    }
}
if ($Publish) {
    $currentFiles = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    foreach ($path in Get-WorkspaceFiles $KitRoot) { if (Test-Watched $path) { [void]$currentFiles.Add($path) } }
    if ($currentFiles.Count -ne $snapshot.Count -or @($snapshot | Where-Object { -not $currentFiles.Contains($_.Path) }).Count) {
        throw 'Checkout changed during validation; source file inventory differs and generated outputs were not published.'
    }
    foreach ($file in $snapshot) {
        if (-not (Test-Path -LiteralPath $file.Path -PathType Leaf) -or (Get-FileHash -LiteralPath $file.Path -Algorithm SHA256).Hash -cne $file.Hash) {
            throw "Checkout changed during validation; generated outputs were not published: $($file.Path)"
        }
    }
    $changed = 0
    foreach ($row in $rows) {
        if (Set-KitGeneratedFile -Source $row.Output -Destination $row.Original) { $changed++ }
    }
    Write-Host "Published $changed changed owners; $($rows.Count - $changed) unchanged."
    if ($Incremental) {
        foreach ($row in $rows) { Set-Content -LiteralPath (Join-Path $keyStore "$($row.Member -replace '[\\/]', '__').key") -Value "$($ownerKeys[$row.Member]) $((Get-FileHash -LiteralPath $row.Original -Algorithm SHA256).Hash)" }
        Write-Host "Recorded input keys for $($rows.Count) owners in $keyStore"
    }
}
if ($KeepStaged) {
    Write-Host "KIT regeneration passed for $($rows.Count) selected owners. Staged workspace: $staged"
} else {
    Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "KIT regeneration passed for $($rows.Count) selected owners. Staged workspace removed: $work"
}
foreach ($lease in $publishLeases) { foreach ($try in 1..10) { Remove-Item -LiteralPath $lease -ErrorAction SilentlyContinue; if (-not (Test-Path -LiteralPath $lease)) { break }; Start-Sleep -Milliseconds 200 } }
