# Test: Hash verification (-v and -c)

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_hash_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$File1 = Join-Path $TestDir "file1.txt"
$File2 = Join-Path $TestDir "file2.txt"
[System.IO.File]::WriteAllText($File1, "hello")
[System.IO.File]::WriteAllText($File2, "world")

# Hashes for "hello" and "world":
# hello: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
# world: 486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7

# --- Test 1: Single Verification Success ---
Write-Host "Testing single hash verification success..."
& $Executable hash -a sha256 -v 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824 $File1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL: Single verification returned error exit code $LASTEXITCODE."
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 2: Single Verification Failure ---
Write-Host "Testing single hash verification failure..."
$ErrorOut = & $Executable hash -a sha256 -v badhashbadhashbadhashbadhash $File1 2>&1 | Out-String
if ($LASTEXITCODE -eq 0) {
    Write-Host "FAIL: Single verification succeeded with a bad hash."
    Remove-Item -Recurse -Force $TestDir
    exit 1
} else {
    Write-Host "PASS: Single verification correctly failed with a bad hash."
}

# --- Test 3: Checksum File Verification Success ---
Write-Host "Testing checksum file verification..."
$ChecksumFile = Join-Path $TestDir "checksums.txt"
# Format of checksum file: hash  path (we must resolve relative paths relative to current work dir)
$Content = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  $File1`n486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7  $File2"
[System.IO.File]::WriteAllText($ChecksumFile, $Content)

& $Executable hash -c $ChecksumFile | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL: Checksum file verification failed with exit code $LASTEXITCODE."
    Remove-Item -Recurse -Force $TestDir
    exit 1
} else {
    Write-Host "PASS: Checksum file verification succeeded."
}

# --- Test 4: Checksum File Verification Failure ---
Write-Host "Testing checksum file verification failure..."
$BadChecksumFile = Join-Path $TestDir "bad_checksums.txt"
$BadContent = "badhash  $File1`n486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7  $File2"
[System.IO.File]::WriteAllText($BadChecksumFile, $BadContent)

& $Executable hash -c $BadChecksumFile | Out-String
if ($LASTEXITCODE -eq 0) {
    Write-Host "FAIL: Checksum file verification did not fail with a bad hash."
    Remove-Item -Recurse -Force $TestDir
    exit 1
} else {
    Write-Host "PASS: Checksum file verification correctly failed on bad hash."
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir
Write-Host "ALL HASH VERIFICATION TESTS PASSED"
exit 0
