param([string]$Kit = (Join-Path $PSScriptRoot '..\src\target\debug\kit.exe'), [int]$WithinMs = 10000)
$ErrorActionPreference = 'Stop'
$home_ = Join-Path ([IO.Path]::GetTempPath()) "kit-inherit-check-$PID"
$argsFile = Join-Path ([IO.Path]::GetTempPath()) "kit-inherit-check-$PID.args"
[IO.File]::WriteAllLines($argsFile, @('job', 'start', 'inherit-check', '60000', 'pwsh', '-NoProfile', '-Command', 'Start-Sleep -Seconds 20; exit 0'))
$info = [Diagnostics.ProcessStartInfo]::new((Resolve-Path $Kit).Path)
$info.ArgumentList.Add('--args-from'); $info.ArgumentList.Add($argsFile)
$info.RedirectStandardOutput = $true; $info.RedirectStandardError = $true; $info.UseShellExecute = $false
$info.Environment['KIT_JOBS_HOME'] = $home_
$watch = [Diagnostics.Stopwatch]::StartNew()
$process = [Diagnostics.Process]::Start($info)
$stdout = $process.StandardOutput.ReadToEndAsync(); $stderr = $process.StandardError.ReadToEndAsync()
$closed = [Threading.Tasks.Task]::WaitAll(@($stdout, $stderr), $WithinMs)
$elapsed = $watch.ElapsedMilliseconds
try {
    if ($closed) {
        "PASS captured stdout and stderr closed after $elapsed ms (job keeps running detached)"
        $stdout.Result.Trim()
        exit 0
    }
    "FAIL captured pipes still open after $elapsed ms: the supervisor inherited them"
    exit 1
} finally {
    Remove-Item $argsFile -ErrorAction SilentlyContinue
}
