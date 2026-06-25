# Test: Default behavior creates a folder when no extension is present.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_create_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# --- Test ---
Write-Host "Running test: ir create new_folder"
& $Executable create "new_folder"

# --- Verification ---
$Result = 1
if (Test-Path "new_folder" -PathType Container) {
    Write-Host "✅ PASS: Directory 'new_folder' was created successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Directory 'new_folder' was not created."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
