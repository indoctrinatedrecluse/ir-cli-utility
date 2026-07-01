# Test: uuid action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: Default UUIDv4 ---
Write-Host "Testing default ir uuid..."
$Output = & $Executable uuid | Out-String
$Output = $Output.Trim()
if ($Output -match "^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$") {
    Write-Host "PASS: Successfully generated standard UUIDv4 ($Output)."
} else {
    Write-Host "FAIL: UUIDv4 format mismatch: '$Output'"
    exit 1
}

# --- Test 2: Generate Multiple UUIDs ---
Write-Host "Testing ir uuid -c 5..."
$Multiple = & $Executable uuid -c 5 | Out-String
$Lines = $Multiple.Trim().Split("`n") | ForEach-Object { $_.Trim() }
if ($Lines.Count -eq 5) {
    Write-Host "PASS: Generated exactly 5 UUIDs."
} else {
    Write-Host "FAIL: Expected 5 lines, got $($Lines.Count)."
    exit 1
}

# --- Test 3: No hyphens and uppercase ---
Write-Host "Testing ir uuid -n -u..."
$NoHyphens = & $Executable uuid -n -u | Out-String
$NoHyphens = $NoHyphens.Trim()
if ($NoHyphens -match "^[0-9A-F]{32}$") {
    Write-Host "PASS: Successfully generated compact uppercase UUID ($NoHyphens)."
} else {
    Write-Host "FAIL: Compact uppercase UUID mismatch: '$NoHyphens'"
    exit 1
}

# --- Test 4: UUIDv7 ---
Write-Host "Testing ir uuid -v 7..."
$UuidV7 = & $Executable uuid -v 7 | Out-String
$UuidV7 = $UuidV7.Trim()
if ($UuidV7 -match "^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$") {
    Write-Host "PASS: Successfully generated standard UUIDv7 ($UuidV7)."
} else {
    Write-Host "FAIL: UUIDv7 format mismatch: '$UuidV7'"
    exit 1
}

# --- Test 5: Invalid version should fail ---
Write-Host "Testing invalid UUID version fails..."
& $Executable uuid -v 5 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: Specifying version 5 failed correctly."
} else {
    Write-Host "FAIL: Specifying version 5 did not return error."
    exit 1
}

Write-Host "ALL UUID TESTS PASSED"
exit 0
