# Test: dns action error handling

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
$Executable = ".\target\debug\ir.exe"

# Test 1: Invalid hostname (labels > 63 chars)
Write-Host "Testing dns with extremely long label..."
$LongLabel = ("a" * 70) + ".com"
$ErrOut = & $Executable dns $LongLabel 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "too long") {
    Write-Host "PASS: Too long label rejected correctly."
} else {
    Write-Host "FAIL: Expected error for too long label. Output: $ErrOut"
    exit 1
}

# Test 2: Non-existent domain
Write-Host "Testing dns with non-existent domain..."
$ErrOut = & $Executable dns thisdomaindoesnotexistatall12345.xyz 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "Failed to resolve records") {
    Write-Host "PASS: Non-existent domain lookup failed correctly."
} else {
    Write-Host "FAIL: Expected failure for non-existent domain. Output: $ErrOut"
    exit 1
}

# Test 3: Multiple arguments
Write-Host "Testing dns with multiple arguments..."
$ErrOut = & $Executable dns google.com extra 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "requires exactly one") {
    Write-Host "PASS: Multiple positional arguments rejected correctly."
} else {
    Write-Host "FAIL: Expected error for multiple arguments. Output: $ErrOut"
    exit 1
}

Write-Host "ALL DNS ERROR TESTS PASSED"
exit 0
