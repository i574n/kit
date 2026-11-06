param([string]$Kit = 'C:\home\git\eoie\src\target\debug\kit.exe')
$ErrorActionPreference = 'Stop'
$root = Join-Path $env:TEMP "kit-read-e2e-$PID"
Remove-Item -Recurse -Force $root -ErrorAction SilentlyContinue
$files = [ordered]@{
    'out/a/main.rs' = "fn f() {`n    if v0 { RecursiveCreate(1) }`n}`n"
    'out/b/main.rs' = "fn g() {}`n"
    'out/c/main.c' = "if v0 {}`n"
    'logs/a.log' = "start a`nstep 1`ndone`n"
    'logs/b.log' = "start b`nfail`n"
    'src/lib.fs' = "let a = 1`nlet b = 2`n// section: parse`nlet parse x =`n    x + 1`nlet c = 3`n"
}
foreach ($entry in $files.GetEnumerator()) {
    $path = Join-Path $root $entry.Key
    New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
    [IO.File]::WriteAllText($path, $entry.Value)
}
$cases = @(
    @{ Name = 'grep with context'; Args = @('grep', '.', 'RecursiveCreate', '--glob', '*.rs', '--context', '1'); Code = 0; Expected = @(
        'out/a/main.rs', '1  fn f() {', '2>     if v0 { RecursiveCreate(1) }', '3  }', 'ok  files 2 searched  1 matched  matches 1  <elapsed>  <clock>') }
    @{ Name = 'grep clipped by --max'; Args = @('grep', '.', 'let', '--max', '2'); Code = 0; Expected = @(
        'src/lib.fs', '1> let a = 1', '2> let b = 2', 'ok  files 6 searched  1 matched  matches 4  shown 2  <elapsed>  <clock>') }
    @{ Name = 'lines table'; Args = @('lines', 'logs/*.log'); Code = 0; Expected = @(
        'path        bytes  lines', 'logs/a.log  20     3', 'logs/b.log  13     2', 'ok  files 2  bytes 33  lines 5  <elapsed>  <clock>') }
    @{ Name = 'read range'; Args = @('read', 'src/lib.fs:3-4'); Code = 0; Expected = @(
        'src/lib.fs (lines 3-4 of 6)', '3  // section: parse', '4  let parse x =', 'ok  files 1  lines 2  <elapsed>  <clock>') }
    @{ Name = 'read missing file fails'; Args = @('read', 'nope.txt'); Code = 2; Expected = @(
        'nope.txt (unreadable: <error>)', 'failed  files 1  unreadable 1  lines 0  <elapsed>  <clock>') }
)
$failures = 0
Push-Location $root
try {
    foreach ($case in $cases) {
        $output = @(& $Kit @($case.Args) 2>&1 | ForEach-Object { "$_" })
        $code = $LASTEXITCODE
        $normalized = @($output | ForEach-Object {
            $_ -replace '  \d+(\.\d)?m?s  \d\d:\d\d:\d\d([+-]\d\d:\d\d|Z)$', '  <elapsed>  <clock>' -replace '\(unreadable: .+\)$', '(unreadable: <error>)'
        })
        $same = ($code -eq $case.Code) -and (($normalized -join "`n") -ceq ($case.Expected -join "`n"))
        if ($same) { "PASS $($case.Name)" } else {
            $failures++
            "FAIL $($case.Name) (exit $code, expected $($case.Code))"
            $output | ForEach-Object { "  got | $_" }
            $case.Expected | ForEach-Object { "  want| $_" }
        }
    }
} finally { Pop-Location; Remove-Item -Recurse -Force $root -ErrorAction SilentlyContinue }
"read e2e: $($cases.Count - $failures)/$($cases.Count) passed"
exit $failures
