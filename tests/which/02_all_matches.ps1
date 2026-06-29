# Test: which -a prints all matches in PATH order.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_which_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path "$TestDir\one" | Out-Null
New-Item -ItemType Directory -Path "$TestDir\two" | Out-Null
New-Item -ItemType File -Path "$TestDir\one\samplecmd.exe" | Out-Null
New-Item -ItemType File -Path "$TestDir\two\samplecmd.exe" | Out-Null
$OldPath = $env:PATH
$OldPathExt = $env:PATHEXT
$env:PATH = (Resolve-Path "$TestDir\one").Path + [IO.Path]::PathSeparator + (Resolve-Path "$TestDir\two").Path
$env:PATHEXT = ".EXE;.BAT;.CMD"

# --- Test ---
Write-Host "Running test: ir which -a samplecmd"
$Output = & $Executable which -a samplecmd

# --- Verification ---
$Result = 1
if (($Output.Count -eq 2) -and ($Output[0] -like "*one*samplecmd.exe") -and ($Output[1] -like "*two*samplecmd.exe")) {
    Write-Host "PASS: which -a printed all matches in PATH order."
    $Result = 0
} else {
    Write-Host "FAIL: which -a did not print expected matches."
    Write-Host "Output was:"
    $Output | ForEach-Object { Write-Host $_ }
}

# --- Teardown ---
$env:PATH = $OldPath
$env:PATHEXT = $OldPathExt
Remove-Item -Recurse -Force $TestDir

exit $Result
