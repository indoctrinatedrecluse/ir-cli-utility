# Test: Create a simple zip archive from a single file.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_archive_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

Set-Content -Path "test_file.txt" -Value "This is a test file."

# --- Test ---
Write-Host "Running test: ir archive test_file.txt --format zip"
& $Executable archive "test_file.txt" --format "zip"

# --- Verification ---
$Result = 1
if (Test-Path "test_file.zip") {
    Write-Host "✅ PASS: Archive 'test_file.zip' was created successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Archive 'test_file.zip' was not created."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
