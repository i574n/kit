param([Parameter(Mandatory)][string] $Label, [Parameter(Mandatory)][string] $Script, [string] $Project = 'C:\home\remote-stage\iggy-m2', [string] $Distro = 'Ubuntu', [Parameter(ValueFromRemainingArguments)][string[]] $Rest = @())
$ErrorActionPreference = 'Stop'
$encoded = (wsl.exe -d $Distro -u root -- sh -c "k3s kubectl -n iggy get secret iggy-root -o jsonpath='{.data.password}'" | Out-String).Trim()
if (-not $encoded) { 'STOP: could not read secret iggy/iggy-root'; exit 1 }
$env:IGGY_PASSWORD = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($encoded))
$env:IGGY_ADDRESS = '127.0.0.1:8090'
& (Join-Path $Project 'target\release\publish.exe') $Label $Script @Rest
exit $LASTEXITCODE
