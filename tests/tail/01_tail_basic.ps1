# Test: Basic tail functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_tail_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "lines.txt"
Set-Content -Path $File -Value @("one", "two", "three", "four", "five")

# --- Test 1: tail -n 2 ---
Write-Host "Running test: ir tail -n 2"
$Output1 = & $Executable tail -n 2 $File | Out-String
$Output1 = $Output1.Trim() -replace "`r`n", "`n"

# --- Test 2: tail -n +3 (starting with 3rd line) ---
Write-Host "Running test: ir tail -n +3"
$Output2 = & $Executable tail -n +3 $File | Out-String
$Output2 = $Output2.Trim() -replace "`r`n", "`n"

# --- Test 3: Conflict check ---
Write-Host "Running test: ir tail -n 2 -c 10 (conflict check)"
$ErrorOutput = & $Executable tail -n 2 -c 10 $File 2>&1 | Out-String

# --- Verification ---
$Result = 0
if ($Output1 -ne "four`nfive") {
    Write-Host "FAIL: tail -n 2 output was incorrect: '$Output1'"
    $Result = 1
} elseif ($Output2 -ne "three`nfour`nfive") {
    Write-Host "FAIL: tail -n +3 output was incorrect: '$Output2'"
    $Result = 1
} elseif ($ErrorOutput -notlike "*cannot be used together*") {
    Write-Host "FAIL: tail conflict check output was incorrect: '$ErrorOutput'"
    $Result = 1
} else {
    Write-Host "PASS: 'tail' successfully sliced file contents and handled conflicts."
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
