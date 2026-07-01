# Test: Basic hashing functions (md5, sha1, sha256, sha512)

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_hash_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$TestFile = Join-Path $TestDir "test.txt"
[System.IO.File]::WriteAllText($TestFile, "Hello World")

# Expected hashes for "Hello World":
# MD5:    b10a8db164e0754105b7a99be72e3fe5
# SHA1:   0a4d55a8d778e5022fab701977c5d840bbc486d0
# SHA256: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e

# --- Test MD5 ---
Write-Host "Testing MD5 hashing..."
$MD5Output = & $Executable hash -a md5 $TestFile | Out-String
if ($MD5Output -like "*b10a8db164e0754105b7a99be72e3fe5*") {
    Write-Host "PASS: MD5 hash match."
} else {
    Write-Host "FAIL: MD5 hash mismatch. Output: $MD5Output"
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test SHA-1 ---
Write-Host "Testing SHA-1 hashing..."
$SHA1Output = & $Executable hash -a sha1 $TestFile | Out-String
if ($SHA1Output -like "*0a4d55a8d778e5022fab701977c5d840bbc486d0*") {
    Write-Host "PASS: SHA-1 hash match."
} else {
    Write-Host "FAIL: SHA-1 hash mismatch. Output: $SHA1Output"
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test SHA-256 (Default) ---
Write-Host "Testing SHA-256 default hashing..."
$SHA256Output = & $Executable hash $TestFile | Out-String
if ($SHA256Output -like "*a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e*") {
    Write-Host "PASS: SHA-256 hash match."
} else {
    Write-Host "FAIL: SHA-256 hash mismatch. Output: $SHA256Output"
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir
Write-Host "ALL BASIC HASH TESTS PASSED"
exit 0
