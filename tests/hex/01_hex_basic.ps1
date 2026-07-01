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

# --- Test 3: Hex empty file ---
Write-Host "Testing ir hex on empty file..."
$EmptyFile = Join-Path $TestDir "empty.txt"
New-Item -ItemType File -Path $EmptyFile | Out-Null
$EmptyOutput = & $Executable hex $EmptyFile | Out-String
if ($LASTEXITCODE -eq 0 -and $EmptyOutput.Trim().Length -eq 0) {
    Write-Host "PASS: Hex dump of empty file returned empty output."
} else {
    Write-Host "FAIL: Hex dump of empty file was not empty. Output:"
    Write-Host $EmptyOutput
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 4: Hex non-existent file fails ---
Write-Host "Testing ir hex on non-existent file..."
& $Executable hex (Join-Path $TestDir "nonexistent.bin") 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: Hex dump correctly failed for non-existent file."
} else {
    Write-Host "FAIL: Hex dump succeeded or exited 0 for non-existent file."
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 5: Hex with custom columns ---
Write-Host "Testing ir hex -c 8..."
$ColsOutput = & $Executable hex -c 8 $TestFile | Out-String
# Each row should now display only 8 bytes. Check that offset 00000008 is present!
if ($ColsOutput -like "*00000008*") {
    Write-Host "PASS: Hex dump custom columns switch (-c) works."
} else {
    Write-Host "FAIL: Hex dump custom columns mismatch. Output:"
    Write-Host $ColsOutput
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir
Write-Host "ALL HEX TESTS PASSED"
exit 0
