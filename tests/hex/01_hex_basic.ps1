# Test: hex dump action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_hex_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$TestFile = Join-Path $TestDir "sample.txt"
[System.IO.File]::WriteAllText($TestFile, "Hello World!")

# --- Test 1: Full Hex Dump ---
Write-Host "Testing ir hex..."
$Output = & $Executable hex $TestFile | Out-String
# Expected output includes offset and hex representation:
# 00000000  48 65 6c 6c 6f 20 57 6f  72 6c 64 21              |Hello World!|
if ($Output -like "*00000000*48 65 6c 6c 6f*|Hello World!|*") {
    Write-Host "PASS: Hex dump format matches."
} else {
    Write-Host "FAIL: Hex dump format mismatch. Output:"
    Write-Host $Output
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 2: Limit Hex Dump ---
Write-Host "Testing ir hex -n 5..."
$LimitOutput = & $Executable hex -n 5 $TestFile | Out-String
if ($LimitOutput -like "*48 65 6c 6c 6f*|Hello|*") {
    Write-Host "PASS: Hex dump limit switch (-n) works."
} else {
    Write-Host "FAIL: Hex dump limit switch mismatch. Output:"
    Write-Host $LimitOutput
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir
Write-Host "ALL HEX TESTS PASSED"
exit 0
