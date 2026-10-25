param(
    [string]$Spec = (Join-Path $PSScriptRoot '..\spec\one-call.md'),
    [string]$Eoie = (Join-Path $PSScriptRoot '..\src\target\debug\eoie.exe'),
    [int[]]$Case = @(),
    [switch]$Quiet
)
$ErrorActionPreference = 'Stop'
$text = [IO.File]::ReadAllText((Resolve-Path $Spec)).Replace("`r`n", "`n")
$headings = [regex]::Matches($text, '(?m)^## (\d+)\. (.+)$')
$placeholders = [ordered]@{
    '<elapsed>' = '\d+(?:\.\d)?m?s'
    '<clock>' = '\d\d:\d\d:\d\d(?:[+-]\d\d:\d\d|Z)'
    '<hash>' = '[0-9a-f]{64}'
    '<commit>' = '[0-9a-f]{7,40}'
    '<pid>' = '\d+'
    '<ms>' = '\d+'
    '<HH:mm>' = '\d\d:\d\d'
}
function Get-Block([string]$Body, [string]$Name) {
    $match = [regex]::Match($Body, '(?ms)^```' + $Name + '\n(.*?)^```')
    if ($match.Success) { $match.Groups[1].Value } else { $null }
}
function Test-PathLine([string]$Line) {
    $Line -notmatch '\s' -and ($Line.Contains('/') -or $Line -match '\.[A-Za-z]+$')
}
function New-Fixture([string]$Root, [string]$Block) {
    $current = $null; $content = [Text.StringBuilder]::new()
    $flush = {
        if ($current) {
            $path = Join-Path $Root $current
            New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
            [IO.File]::WriteAllText($path, $content.ToString())
        }
    }
    foreach ($line in ($Block.TrimEnd("`n") -split "`n")) {
        if (Test-PathLine $line) { & $flush; $current = $line; $content = [Text.StringBuilder]::new() }
        else { [void]$content.Append($line + "`n") }
    }
    & $flush
}
function Get-CommandArguments([string]$Command) {
    $match = [regex]::Match($Command.Trim(), "(?s)^eoie (\S+) (\S+) '(.*)'$")
    if (-not $match.Success) { return $null }
    @($match.Groups[1].Value, $match.Groups[2].Value, $match.Groups[3].Value)
}
function Get-DirectArguments([string]$Command) {
    $trimmed = $Command.Trim()
    if ($trimmed -notmatch '^eoie\s' -or $trimmed.Contains("`n")) { return $null }
    @([regex]::Matches($trimmed.Substring(5), "'([^']*)'|`"([^`"]*)`"|(\S+)") | ForEach-Object { if ($_.Groups[1].Success) { $_.Groups[1].Value } elseif ($_.Groups[2].Success) { $_.Groups[2].Value } else { $_.Groups[3].Value } })
}
function Get-SetupPrograms([string]$Program, [switch]$Ungrouped) {
    $statements = [Collections.Generic.List[string]]::new()
    foreach ($line in ($Program -split "`n")) {
        if (-not $line.Trim()) { continue }
        if ($line -match '^\s' -and $statements.Count) { $statements[$statements.Count - 1] += "`n" + $line; continue }
        $quoted = $false; $current = [Text.StringBuilder]::new()
        foreach ($character in $line.ToCharArray()) {
            if ($character -eq '"') { $quoted = -not $quoted }
            if ($character -eq ';' -and -not $quoted) { if ($current.ToString().Trim()) { $statements.Add($current.ToString().Trim()) }; $current = [Text.StringBuilder]::new() }
            else { [void]$current.Append($character) }
        }
        if ($current.ToString().Trim()) { $statements.Add($current.ToString().Trim()) }
    }
    if ($Ungrouped) { return $statements }
    $family = { param($statement) switch -regex ($statement) { '^git\s' { 'git' } '^(append|copy)\s' { 'write' } default { 'other' } } }
    $groups = [Collections.Generic.List[string]]::new(); $last = $null
    foreach ($statement in $statements) {
        $name = & $family $statement
        if ($name -eq $last) { $groups[$groups.Count - 1] += "`n" + $statement } else { $groups.Add($statement); $last = $name }
    }
    $groups
}
function ConvertTo-Pattern([string]$Expected) {
    $escaped = [regex]::Escape($Expected.TrimEnd("`n"))
    foreach ($entry in $placeholders.GetEnumerator()) {
        $escaped = [regex]::Replace($escaped, [regex]::Escape([regex]::Escape($entry.Key)) + '(?:\\ ){2,}', { param($cell) $entry.Value + '\ {2,}' })
        $escaped = $escaped.Replace([regex]::Escape($entry.Key), $entry.Value)
    }
    '^' + $escaped + '$'
}
$results = @()
for ($index = 0; $index -lt $headings.Count; $index++) {
    $number = [int]$headings[$index].Groups[1].Value
    if ($Case.Count -and $Case -notcontains $number) { continue }
    $start = $headings[$index].Index
    $end = if ($index + 1 -lt $headings.Count) { $headings[$index + 1].Index } else { $text.Length }
    $body = $text.Substring($start, $end - $start)
    $fixture = Get-Block $body 'fixture'; $command = Get-Block $body 'command'; $expected = Get-Block $body 'expected'
    if ($null -eq $command -or $null -eq $expected) { continue }
    $arguments = Get-CommandArguments $command
    if (-not $arguments) { $arguments = Get-DirectArguments $command }
    $root = Join-Path ([IO.Path]::GetTempPath()) "eoie-one-call-$PID-$number"
    Remove-Item -Recurse -Force $root -ErrorAction SilentlyContinue
    New-Item -ItemType Directory -Force $root | Out-Null
    try {
        if ($fixture) { New-Fixture $root $fixture }
        if (-not $arguments) { $results += [pscustomobject]@{ Case = $number; Verdict = 'red'; Reason = 'command is not eoie <verb> <root> ''<program>'''; Output = '' }; continue }
        Push-Location $root
        $callerEnvironment = @(Get-ChildItem env: | Where-Object { $_.Name -like 'EOIE_*' -or $_.Name -like 'SPIRAL_*' } | ForEach-Object { [pscustomobject]@{ Name = $_.Name; Value = $_.Value } })
        foreach ($saved in $callerEnvironment) { Remove-Item "env:$($saved.Name)" }
        $env:EOIE_JOBS_HOME = "$root-jobs"
        $env:EOIE_WINDOWS_HOME = "$root-windows"
        $setupFailure = ''
        $setup = Get-Block $body 'setup'
        $setupArguments = if ($setup) { Get-CommandArguments $(if ($setup.Trim() -match '^eoie\s') { $setup } else { "eoie $($setup.Trim())" }) } else { $null }
        if ($setup -and -not $setupArguments) {
            foreach ($statement in (Get-SetupPrograms $setup -Ungrouped)) {
                $tokens = @([regex]::Matches($statement, '"([^"]*)"|(\S+)') | ForEach-Object { if ($_.Groups[1].Success) { $_.Groups[1].Value } else { $_.Groups[2].Value } })
                if ($tokens.Count -gt 1 -and $tokens[0] -eq 'eoie') { $tokens = @($tokens[1..($tokens.Count - 1)]) }
                if ($tokens.Count -gt 1 -and $tokens[0] -eq 'eoie') { $tokens = @($tokens[1..($tokens.Count - 1)]) }
                $setupOutput = (& $Eoie @tokens 2>&1 | ForEach-Object { "$_" }) -join ' | '
                if ($LASTEXITCODE -ne 0) { $setupFailure = "setup failed: $setupOutput"; break }
            }
        }
        elseif ($setupArguments) {
            foreach ($program in (Get-SetupPrograms $setupArguments[2])) {
                $setupOutput = (& $Eoie $setupArguments[0] $setupArguments[1] $program 2>&1 | ForEach-Object { "$_" }) -join ' | '
                if ($LASTEXITCODE -ne 0) { $setupFailure = "setup failed: $setupOutput"; break }
            }
        }
        if ($setupFailure) { Pop-Location; $results += [pscustomobject]@{ Case = $number; Verdict = 'red'; Reason = $setupFailure; Output = $setupFailure; Expected = $expected }; continue }
        try { $output = (& $Eoie @arguments 2>&1 | ForEach-Object { "$_" }) -join "`n" } finally { Pop-Location }
        $green = [regex]::IsMatch($output, (ConvertTo-Pattern $expected))
        $reason = if ($green) { '' } else {
            $got = @($output -split "`n")
            $want = @($expected.TrimEnd("`n") -split "`n")
            $first = @(0..([Math]::Max($got.Count, $want.Count) - 1) | Where-Object { $_ -ge $want.Count -or $_ -ge $got.Count -or -not [regex]::IsMatch($got[$_], (ConvertTo-Pattern $want[$_])) } | Select-Object -First 1)
            if ($first.Count) { $at = $first[0]; "line $($at + 1): got '$(if ($at -lt $got.Count) { $got[$at] } else { '<none>' })' want '$(if ($at -lt $want.Count) { $want[$at] } else { '<none>' })'" } else { $got[0] }
        }
        $results += [pscustomobject]@{ Case = $number; Verdict = $(if ($green) { 'green' } else { 'red' }); Reason = $reason; Output = $output; Expected = $expected }
    } finally {
        Remove-Item -Recurse -Force $root, "$root-jobs", "$root-windows" -ErrorAction SilentlyContinue
        foreach ($name in @(Get-ChildItem env: | Where-Object { $_.Name -like 'EOIE_*' -or $_.Name -like 'SPIRAL_*' } | ForEach-Object Name)) { Remove-Item "env:$name" }
        foreach ($saved in @($callerEnvironment)) { if ($saved) { Set-Item "env:$($saved.Name)" $saved.Value } }
    }
}
foreach ($result in $results) {
    "{0,3} {1,-5} {2}" -f $result.Case, $result.Verdict, $result.Reason
    if (-not $Quiet -and $result.Verdict -eq 'red' -and $Case.Count) {
        ($result.Output -split "`n") | ForEach-Object { "      got | $_" }
        ($result.Expected.TrimEnd("`n") -split "`n") | ForEach-Object { "     want | $_" }
    }
}
$green = @($results | Where-Object Verdict -eq 'green').Count
"one-call spec: $green/$($results.Count) green"
