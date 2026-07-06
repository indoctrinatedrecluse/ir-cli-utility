# Test: math action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"

# --- Test 1: Basic addition and multiplication (PEMDAS) ---
Write-Host "Testing basic math evaluation..."
$Output = & $Executable math "2 * (3.5 + 4)" | Out-String
if ($Output.Trim() -eq "15") {
    Write-Host "PASS: Evaluated 2 * (3.5 + 4) correctly."
} else {
    Write-Host "FAIL: Output mismatch: '$($Output.Trim())'"
    exit 1
}

# --- Test 2: Modulo operator ---
Write-Host "Testing modulo operator..."
$ModOut = & $Executable math "10 % 3" | Out-String
if ($ModOut.Trim() -eq "1") {
    Write-Host "PASS: Evaluated 10 % 3 correctly."
} else {
    Write-Host "FAIL: Modulo mismatch: '$($ModOut.Trim())'"
    exit 1
}

# --- Test 3: Power operator (Right-associative) ---
Write-Host "Testing power operator..."
$PowOut = & $Executable math "2^3^2" | Out-String
if ($PowOut.Trim() -eq "512") {
    Write-Host "PASS: Evaluated 2^3^2 correctly."
} else {
    Write-Host "FAIL: Power mismatch: '$($PowOut.Trim())'"
    exit 1
}

# --- Test 4: Floating point division ---
Write-Host "Testing floating point division..."
$DivOut = & $Executable math "7 / 2" | Out-String
if ($DivOut.Trim() -eq "3.5") {
    Write-Host "PASS: Evaluated 7 / 2 correctly."
} else {
    Write-Host "FAIL: Float division mismatch: '$($DivOut.Trim())'"
    exit 1
}

# --- Test 5: Division by zero should fail ---
Write-Host "Testing division by zero fails..."
& $Executable math "10 / 0" 2>$null | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: Division by zero failed correctly."
} else {
    Write-Host "FAIL: Division by zero did not return error code."
    exit 1
}

# --- Test 6: Syntax error should fail ---
Write-Host "Testing syntax error fails..."
& $Executable math "2 + * 3" 2>$null | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: Syntax error failed correctly."
} else {
    Write-Host "FAIL: Syntax error did not return error code."
    exit 1
}

# --- Test 7: Math functions and constants ---
Write-Host "Testing math functions and constants..."
$FuncOut = & $Executable math "sqrt(144) + sin(pi / 2)" | Out-String
if ($FuncOut.Trim() -eq "13") {
    Write-Host "PASS: Evaluated functions and constants correctly."
} else {
    Write-Host "FAIL: Function evaluation mismatch: '$($FuncOut.Trim())'"
    exit 1
}

# --- Test 8: Variable assignment evaluation ---
Write-Host "Testing variable assignment evaluation..."
$AssignOut = & $Executable math "x = 4.5 * 2" | Out-String
if ($AssignOut.Trim() -eq "9") {
    Write-Host "PASS: Evaluated assignment statement correctly."
} else {
    Write-Host "FAIL: Assignment evaluation mismatch: '$($AssignOut.Trim())'"
    exit 1
}

Write-Host "ALL MATH TESTS PASSED"
exit 0
