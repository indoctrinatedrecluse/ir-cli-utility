# Test: Basic tee functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_tee_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File1 = Join-Path $TestDir "file1.txt"
$File2 = Join-Path $TestDir "file2.txt"

# --- Test ---
Write-Host "Running test: echo 'hello world' | ir tee file1 file2"
$Output = "hello world" | & $Executable tee $File1 $File2 | Out-String

# --- Verification ---
$Result = 0
$Content1 = Get-Content $File1 -ErrorAction SilentlyContinue
$Content2 = Get-Content $File2 -ErrorAction SilentlyContinue

if ($Output.Trim() -ne "hello world") {
    Write-Host "FAIL: tee stdout was incorrect: '$Output'"
    $Result = 1
} elseif ($Content1 -ne "hello world") {
    Write-Host "FAIL: file1 content was incorrect: '$Content1'"
    $Result = 1
} elseif ($Content2 -ne "hello world") {
    Write-Host "FAIL: file2 content was incorrect: '$Content2'"
    $Result = 1
} else {
    Write-Host "PASS: 'tee' successfully replicated stdin to stdout and files."
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
