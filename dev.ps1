[CmdletBinding(DefaultParameterSetName = 'Run')]
param(
    [Parameter(Mandatory, ParameterSetName = 'Run')][string[]]$Package,
    [Parameter(Mandatory, ParameterSetName = 'List')][switch]$List,
    [switch]$CheckOnly,
    [switch]$Publish,
    [switch]$Offline,
    [string]$CompilerBundle = $env:KIT_SPIRAL_BUNDLE,
    [ValidateRange(1, 600)][int]$TimeoutSec = 180
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'compiler-contracts/workspace.ps1')
if ($List) {
    Get-KitOwners -Root $PSScriptRoot -IncludeAuxiliary | Select-Object Package, Member, Input, Target
    return
}
$Package = @($Package | ForEach-Object { $_ -split ',' } | ForEach-Object Trim | Where-Object { $_ })
$Package = @(Get-KitOwners -Root $PSScriptRoot -Package $Package -IncludeAuxiliary | ForEach-Object Package | Select-Object -Unique)
& (Join-Path $PSScriptRoot 'compiler-contracts/test-regeneration.ps1') `
    -KitRoot $PSScriptRoot -Package $Package -CompilerBundle $CompilerBundle -IncludeAuxiliary `
    -TargetDirectory (Join-Path $PSScriptRoot 'src/target/dev-validation') `
    -TimeoutSec $TimeoutSec -CargoCheck -Test:(-not $CheckOnly) -SkipRelease `
    -Publish:$Publish -Offline:$Offline
