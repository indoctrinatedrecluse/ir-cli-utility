# Test: ip action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: Local adapter listing ---
Write-Host "Testing local adapter listing..."
$Output = & $Executable ip | Out-String
if ($Output -like "*Status:*" -or $Output -like "*MAC Address:*") {
    Write-Host "PASS: Local network adapters listed successfully."
} else {
    Write-Host "FAIL: Local network adapter output format mismatch. Output:"
    Write-Host $Output
    exit 1
}

# --- Test 2: Public IP lookup ---
Write-Host "Testing public IP lookup..."
$Public = & $Executable ip -p | Out-String
if ($Public -like "*Public IP:*" -and $Public -like "*Location:*") {
    Write-Host "PASS: Public IP query details match expected format."
} else {
    Write-Host "FAIL: Public IP query format mismatch. Output:"
    Write-Host $Public
    exit 1
}

# --- Test 3: Invalid switch fails ---
Write-Host "Testing invalid switch fails..."
& $Executable ip -z 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: Specifying invalid switch failed correctly."
} else {
    Write-Host "FAIL: Specifying invalid switch did not return error."
    exit 1
}

Write-Host "ALL IP TESTS PASSED"
exit 0
