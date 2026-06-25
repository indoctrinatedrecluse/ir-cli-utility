# Test: Force remove of a non-empty directory.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_remove_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType Directory -Path "dir_to_remove\subdir" | Out-Null
New-Item -ItemType File -Path "dir_to_remove\file.txt" | Out-Null

# --- Test ---
Write-Host "Running test: ir remove -f dir_to_remove"
& $Executable remove -f "dir_to_remove"

# --- Verification ---
$Result = 1
if (-not (Test-Path "dir_to_remove")) {
    Write-Host "✅ PASS: Non-empty directory was force-removed successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Directory still exists after force remove operation."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
