# Test: Extract a zip archive.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_archive_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create a dummy archive
$ZipFile = [System.IO.Compression.ZipFile]::CreateFromDirectory($PWD, "test.zip")
$ZipFile.Dispose()

# --- Test ---
Write-Host "Running test: ir archive --unarc test.zip"
& $Executable archive --unarc "test.zip"

# --- Verification ---
$Result = 1
if (Test-Path "file1.txt") {
    Write-Host "✅ PASS: File was extracted successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Extracted file not found."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
