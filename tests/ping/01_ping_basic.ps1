# Test: ping action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test: Ping localhost ---
Write-Host "Testing ir ping 127.0.0.1 -c 2..."
$Output = & $Executable ping 127.0.0.1 -c 2 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL: ir ping failed with exit code $LASTEXITCODE."
    exit 1
}

if ($Output -like "*Reply from 127.0.0.1*bytes=*time=*") {
    Write-Host "PASS: ping response matches expected format."
} else {
    Write-Host "FAIL: ping response mismatch. Output:"
    Write-Host $Output
    exit 1
}

if ($Output -like "*Packets: Sent = 2, Received = 2*") {
    Write-Host "PASS: ping statistics are correct."
} else {
    Write-Host "FAIL: ping statistics mismatch. Output:"
    Write-Host $Output
    exit 1
}

# --- Test 2: Ping non-existent domain should fail ---
Write-Host "Testing ir ping invalid domain..."
& $Executable ping thisdomaindoesnotexistatall.invalid 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: ping correctly failed to resolve non-existent domain."
} else {
    Write-Host "FAIL: ping succeeded or exited 0 for invalid domain."
    exit 1
}

Write-Host "ALL PING TESTS PASSED"
exit 0
