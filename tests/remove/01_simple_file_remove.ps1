# Test: Simple, non-interactive file removal.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_remove_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType File -Name "file_to_remove.txt" | Out-Null

# --- Test ---
Write-Host "Running test: ir remove file_to_remove.txt"
# The -y is added to skip any potential prompts for this basic test
& $Executable remove -y "file_to_remove.txt"

# --- Verification ---
$Result = 1
if (-not (Test-Path "file_to_remove.txt")) {
    Write-Host "✅ PASS: File was removed successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File still exists after remove operation."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
