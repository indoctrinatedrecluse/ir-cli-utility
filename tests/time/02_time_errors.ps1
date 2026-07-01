# Test: time action error handling

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
$Executable = ".\target\debug\ir.exe"

# Test 1: Empty command argument
Write-Host "Testing time with empty command..."
$ErrOut = & $Executable time 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and ($ErrOut -match "requires a command" -or $ErrOut -match "ir-time")) {
    Write-Host "PASS: Empty command rejected correctly."
} else {
    Write-Host "FAIL: Expected error for empty command. Output: $ErrOut"
    exit 1
}

# Test 2: Invalid command that doesn't exist
Write-Host "Testing time with non-existent command..."
$ErrOut = & $Executable time non_existent_command_123 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "Failed to spawn") {
    Write-Host "PASS: Non-existent command failed correctly."
} else {
    Write-Host "FAIL: Expected error for non-existent command. Output: $ErrOut"
    exit 1
}

Write-Host "ALL TIME ERROR TESTS PASSED"
exit 0
