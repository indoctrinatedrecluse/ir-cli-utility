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

# --- Test 3: Fetch output to a file ---
Write-Host "Testing ir fetch -o file.txt..."
$OutputFile = "temp_fetch_out.txt"
if (Test-Path $OutputFile) { Remove-Item -Force $OutputFile }

& $Executable fetch -o $OutputFile https://api.ipify.org | Out-String
if ($LASTEXITCODE -eq 0 -and (Test-Path $OutputFile)) {
    $Content = Get-Content $OutputFile
    if ($Content -match "^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}") {
        Write-Host "PASS: fetch -o successfully wrote download to file."
    } else {
        Write-Host "FAIL: Output file content mismatch. Content: $Content"
        Remove-Item -Force $OutputFile
        exit 1
    }
} else {
    Write-Host "FAIL: fetch -o failed or output file not created."
    if (Test-Path $OutputFile) { Remove-Item -Force $OutputFile }
    exit 1
}
Remove-Item -Force $OutputFile

# --- Test 4: Fetch invalid domain should fail ---
Write-Host "Testing ir fetch invalid domain..."
& $Executable fetch https://thisdomaindoesnotexistatall.invalid 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: fetch failed correctly for non-existent domain."
} else {
    Write-Host "FAIL: fetch succeeded or exited 0 for invalid domain."
    exit 1
}

Write-Host "ALL FETCH TESTS PASSED"
exit 0
