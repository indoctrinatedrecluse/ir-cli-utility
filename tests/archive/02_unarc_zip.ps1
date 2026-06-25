# Test: Extract a zip archive.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_archive_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create a dummy archive using our helper
rustc ..\create_zip.rs
.\create_zip.exe test.zip

# --- Test ---
Write-Host "Running test: ir archive --unarc test.zip"
& $Executable archive --unarc "test.zip"

# --- Verification ---
$Result = 1
if ((Test-Path "file1.txt") -and ((Get-Content "file1.txt") -eq "Hello, world!")) {
    Write-Host "✅ PASS: File was extracted successfully with correct content."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Extracted file not found or content is incorrect."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir
Remove-Item "tests\archive\create_zip.exe"

exit $Result
