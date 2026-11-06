param(
    [Parameter(Mandatory)][string] $Holder,
    [int] $Minutes = 60,
    [int] $HolderPid = 0,
    [switch] $Release
)
$ErrorActionPreference = 'Stop'
$holdDir = Join-Path $env:LOCALAPPDATA 'kit\train-holds'
[void][IO.Directory]::CreateDirectory($holdDir)
$file = Join-Path $holdDir "$($Holder -replace '[^\w.-]', '_').hold"
if ($Release) {
    if (Test-Path -LiteralPath $file) { Remove-Item -LiteralPath $file -Confirm:$false; [Console]::WriteLine("train hold released: $Holder") }
    else { [Console]::WriteLine("no train hold named $Holder") }
    exit 0
}
$until = (Get-Date).AddMinutes($Minutes).ToString('yyyy-MM-ddTHH:mm')
[IO.File]::WriteAllText($file, "holder=$Holder`nuntil=$until`npid=$HolderPid`nsince=$(Get-Date -Format HH:mm)`n")
[Console]::WriteLine("train hold set: $Holder until $until$(if ($HolderPid) { " or until pid $HolderPid ends, whichever is later" }); launch-train.ps1 waits while it is live; release with -Release")
