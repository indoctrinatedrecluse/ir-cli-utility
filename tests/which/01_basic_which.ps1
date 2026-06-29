# Test: which locates a command in PATH.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_which_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
New-Item -ItemType File -Path "$TestDir\samplecmd.exe" | Out-Null
$OldPath = $env:PATH
$OldPathExt = $env:PATHEXT
$env:PATH = (Resolve-Path $TestDir).Path + [IO.Path]::PathSeparator + $env:PATH
$env:PATHEXT = ".EXE;.BAT;.CMD"

# --- Test ---
Write-Host "Running test: ir which samplecmd"
$Output = & $Executable which samplecmd | Out-String

# --- Verification ---
$Result = 1
if ($Output -like "*samplecmd.exe*") {
    Write-Host "PASS: which located the command in PATH."
    $Result = 0
} else {
    Write-Host "FAIL: which did not locate the expected command."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
$env:PATH = $OldPath
$env:PATHEXT = $OldPathExt
Remove-Item -Recurse -Force $TestDir

exit $Result
