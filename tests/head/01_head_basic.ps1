# Test: Basic head functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_head_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "lines.txt"
Set-Content -Path $File -Value @("one", "two", "three", "four", "five")

# --- Test 1: head -n 2 ---
Write-Host "Running test: ir head -n 2"
$Output1 = & $Executable head -n 2 $File | Out-String
$Output1 = $Output1.Trim() -replace "`r`n", "`n"

# --- Test 2: head -n -2 (all but last 2) ---
Write-Host "Running test: ir head -n -2"
$Output2 = & $Executable head -n -2 $File | Out-String
$Output2 = $Output2.Trim() -replace "`r`n", "`n"

# --- Test 3: head -n 2 -c 10 (should fail/conflict) ---
Write-Host "Running test: ir head -n 2 -c 10 (conflict check)"
$ErrorOutput = & $Executable head -n 2 -c 10 $File 2>&1 | Out-String

# --- Verification ---
$Result = 0
if ($Output1 -ne "one`ntwo") {
    Write-Host "FAIL: head -n 2 output was incorrect: '$Output1'"
    $Result = 1
} elseif ($Output2 -ne "one`ntwo`nthree") {
    Write-Host "FAIL: head -n -2 output was incorrect: '$Output2'"
    $Result = 1
} elseif ($ErrorOutput -notlike "*cannot be used together*") {
    Write-Host "FAIL: head conflict check output was incorrect: '$ErrorOutput'"
    $Result = 1
} else {
    Write-Host "PASS: 'head' successfully sliced file contents and handled conflicts."
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
