# Test: Simple file copy to a directory.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_copy_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType File -Name "source_file.txt" | Out-Null
New-Item -ItemType Directory -Name "dest_dir" | Out-Null

# --- Test ---
Write-Host "Running test: ir copy source_file.txt dest_dir"
& $Executable copy "source_file.txt" "dest_dir"

# --- Verification ---
$Result = 1
if (Test-Path "dest_dir/source_file.txt") {
    Write-Host "✅ PASS: File was copied successfully to the destination directory."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File was not found in the destination directory."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
