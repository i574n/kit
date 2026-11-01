if (-not $IsWindows) { return }
$skip = 'Path', 'PSModulePath', 'TEMP', 'TMP'
foreach ($scope in 'Machine', 'User') {
    $vars = [Environment]::GetEnvironmentVariables($scope)
    foreach ($variableName in $vars.Keys) {
        if ($skip -contains $variableName) { continue }
        if ($null -eq [Environment]::GetEnvironmentVariable($variableName, 'Process')) {
            [Environment]::SetEnvironmentVariable($variableName, $vars[$variableName], 'Process')
        }
    }
}
foreach ($sdkName in 'DOTNET_ROOT', 'MSBuildSDKsPath') {
    if ("$([Environment]::GetEnvironmentVariable($sdkName, 'Process'))" -match 'dotnet3-sdk') { [Environment]::SetEnvironmentVariable($sdkName, $null, 'Process') }
}
$machinePath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$env:Path = (@($env:Path -split ';') + @($userPath -split ';') + @($machinePath -split ';') |
    Where-Object { $_ } | Select-Object -Unique) -join ';'
