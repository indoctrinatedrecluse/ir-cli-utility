# Test: sleep action error handling

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
$Executable = ".\target\debug\ir.exe"

# Test 1: Negative duration
Write-Host "Testing sleep with negative duration..."
$ErrOut = & $Executable sleep -5s 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and ($ErrOut -match "negative" -or $ErrOut -match "Unknown switch")) {
    Write-Host "PASS: Negative duration rejected correctly."
} else {
    Write-Host "FAIL: Expected error for negative duration. Output: $ErrOut"
    exit 1
}

# Test 2: Invalid unit suffix
Write-Host "Testing sleep with invalid suffix..."
$ErrOut = & $Executable sleep 5x 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "Unknown unit") {
    Write-Host "PASS: Invalid unit rejected correctly."
} else {
    Write-Host "FAIL: Expected error for invalid unit. Output: $ErrOut"
    exit 1
}

# Test 3: Non-numeric duration
Write-Host "Testing sleep with non-numeric value..."
$ErrOut = & $Executable sleep abc 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "No numeric value") {
    Write-Host "PASS: Non-numeric duration rejected correctly."
} else {
    Write-Host "FAIL: Expected error for non-numeric duration. Output: $ErrOut"
    exit 1
}

# Test 4: Multiple positional arguments
Write-Host "Testing sleep with multiple arguments..."
$ErrOut = & $Executable sleep 5s 10s 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "requires exactly one") {
    Write-Host "PASS: Multiple arguments rejected correctly."
} else {
    Write-Host "FAIL: Expected error for multiple arguments. Output: $ErrOut"
    exit 1
}

# Test 5: Empty arguments
Write-Host "Testing sleep with empty arguments..."
$ErrOut = & $Executable sleep 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "requires exactly one") {
    Write-Host "PASS: Empty arguments rejected correctly."
} else {
    Write-Host "FAIL: Expected error for empty arguments. Output: $ErrOut"
    exit 1
}

Write-Host "ALL SLEEP ERROR TESTS PASSED"
exit 0
