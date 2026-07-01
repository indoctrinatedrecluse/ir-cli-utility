# Test: fetch action retrieves URL contents

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: Fetch plain text IP ---
Write-Host "Testing ir fetch api.ipify.org..."
$Output = & $Executable fetch https://api.ipify.org | Out-String
if ($Output -match "^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}") {
    Write-Host "PASS: fetch successfully retrieved public IP address ($($Output.Trim()))."
} else {
    Write-Host "FAIL: Did not get expected IP address format. Output was:"
    Write-Host $Output
    exit 1
}

# --- Test 2: Fetch headers using -i ---
Write-Host "Testing ir fetch -i api.ipify.org..."
$HeaderOutput = & $Executable fetch -i https://api.ipify.org | Out-String
if ($HeaderOutput -like "*HTTP/1.1 200*") {
    Write-Host "PASS: fetch -i correctly included response headers."
} else {
    Write-Host "FAIL: Response headers missing or mismatch. Output was:"
    Write-Host $HeaderOutput
    exit 1
}

Write-Host "ALL FETCH TESTS PASSED"
exit 0
