[CmdletBinding()]
param(
    [string]$KitRoot = (Join-Path $PSScriptRoot '..'),
    [string]$KitBinary,
    [switch]$RequireReady,
    [switch]$PassThru
)
$ErrorActionPreference = 'Stop'
$KitRoot = (Resolve-Path -LiteralPath $KitRoot).Path
$suffix = if ($IsWindows) { '.exe' } else { '' }
if (-not $KitBinary) { $KitBinary = Join-Path $KitRoot "kit$suffix" }
$KitBinary = (Resolve-Path -LiteralPath $KitBinary).Path
$work = Join-Path $KitRoot ('.cache/strict-preflight/' + [guid]::NewGuid().ToString('N'))
$release = Join-Path $work 'release'
[void][IO.Directory]::CreateDirectory($release)
Copy-Item -LiteralPath $KitBinary -Destination (Join-Path $work "supervisor$suffix")
$KitBinary = Join-Path $work "supervisor$suffix"

$snapshot = & $PSScriptRoot/test-source-package.ps1 -KitRoot $KitRoot -KitBinary $KitBinary -SkipBuild -PassThru | ForEach-Object {
    if ($_.PSObject.Properties.Name -contains 'Manifest') { $_ } else { Write-Host $_ }
}
if (-not $snapshot -or $snapshot.Count -gt 1) { throw 'Expected one verified source snapshot.' }
$manifest = Get-Content -Raw -LiteralPath $snapshot.Manifest | ConvertFrom-Json
$excluded = @($manifest | Where-Object { $_.Path -eq 'src/.gitignore' } | ForEach-Object { $_.Path })
$releaseFiles = @($manifest | Where-Object { $_.Path -match '^(src|state|evidence)/' -and $_.Path -notin $excluded })
$count = 0
foreach ($file in $releaseFiles) {
    $destination = Join-Path $release $file.Path
    [void][IO.Directory]::CreateDirectory((Split-Path $destination -Parent))
    Copy-Item -LiteralPath (Join-Path $snapshot.RestoredRoot $file.Path) -Destination $destination
    if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash -cne $file.SHA256) { throw "Staging hash mismatch: $($file.Path)" }
    $count++
}
[void][IO.Directory]::CreateDirectory((Join-Path $release 'evidence/coverage'))
Copy-Item -LiteralPath $KitBinary -Destination (Join-Path $release "kit$suffix")

$leaseRoot = $env:KIT_LEASE_ROOT
Push-Location -LiteralPath ([IO.Path]::GetTempPath())
try {
    $env:KIT_LEASE_ROOT = $null
    & $KitBinary proxy command-capture $work 'strict-check.txt' 120000 $KitBinary bundle check $release kit | ForEach-Object { Write-Host $_ }
    $captureExit = $LASTEXITCODE
} finally { Pop-Location; $env:KIT_LEASE_ROOT = $leaseRoot }
if ($captureExit -ne 0) { throw 'Strict preflight supervision failed.' }
$receiptPath = Join-Path $work 'strict-check.txt'
$receipt = [IO.File]::ReadAllText($receiptPath)
$headerEnd = $receipt.IndexOf("--- stdout ---" + [char]10, [StringComparison]::Ordinal)
if ($headerEnd -lt 0) { throw 'Strict preflight process receipt is malformed.' }
$header = $receipt.Substring(0, $headerEnd)
$ready = $header -match '(?m)^status=0\r?$' -and $header -match '(?m)^termination=Exited\(0\)\r?$'
$diagnostics = @($receipt -split '\r?\n' | Where-Object { $_ -match '^(bundle error:|kit error:)' })
foreach ($file in $releaseFiles) {
    if ((Get-FileHash -LiteralPath (Join-Path $release $file.Path) -Algorithm SHA256).Hash -cne $file.SHA256) {
        throw "Strict preflight mutated staged source: $($file.Path)"
    }
}
$result = [pscustomobject]@{
    Schema = 1
    SourceManifest = $snapshot.Manifest
    SourceManifestSHA256 = (Get-FileHash -LiteralPath $snapshot.Manifest -Algorithm SHA256).Hash
    KitSHA256 = (Get-FileHash -LiteralPath $KitBinary -Algorithm SHA256).Hash
    DriverSHA256 = (Get-FileHash -LiteralPath $PSCommandPath -Algorithm SHA256).Hash
    ReleaseRoot = $release
    StagedSourceFiles = $count
    RepositoryOnlyFiles = $excluded
    CheckPassed = [bool]$ready
    StrictReleaseCertification = $false
    EvidenceRenewed = $false
    Receipt = $receiptPath
    Diagnostics = $diagnostics
}
$report = Join-Path $work 'report.json'
[IO.File]::WriteAllText($report, ($result | ConvertTo-Json -Depth 5), [Text.UTF8Encoding]::new($false))
Write-Host "KIT strict preflight: ready=$ready diagnostics=$($diagnostics.Count) report=$report"
foreach ($diagnostic in $diagnostics) { Write-Host $diagnostic }
if ($RequireReady -and -not $ready) { throw "Strict release preflight rejected this snapshot; see $report" }
if ($PassThru) { $result }
