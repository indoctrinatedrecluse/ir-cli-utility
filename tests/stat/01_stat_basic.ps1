# Test: Basic stat functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_stat_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "test.txt"
Set-Content -Path $File -Value "stat check"

# --- Test 1: stat file ---
Write-Host "Running test: ir stat file"
$Output1 = & $Executable stat $File | Out-String

# --- Test 2: stat -c "%A %n" file ---
Write-Host "Running test: ir stat -c '%A %n' file"
$Output2 = & $Executable stat -c "%A %n" $File | Out-String

# --- Test 3: Conflict check ---
Write-Host "Running test: ir stat -c '%A' -t file (conflict check)"
$ErrorOutput = & $Executable stat -c "%A" -t $File 2>&1 | Out-String

# --- Verification ---
$Result = 0
if ($Output1 -notlike "*File:*") {
    Write-Host "FAIL: stat output was missing 'File:' header: '$Output1'"
    $Result = 1
} elseif ($Output2.Trim() -notlike "*-rw*test.txt") {
    Write-Host "FAIL: stat custom format output was incorrect: '$Output2'"
    $Result = 1
} elseif ($ErrorOutput -notlike "*cannot be used together*") {
    Write-Host "FAIL: stat conflict check output was incorrect: '$ErrorOutput'"
    $Result = 1
} else {
    Write-Host "PASS: 'stat' successfully retrieved metadata and formatted outputs."
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
