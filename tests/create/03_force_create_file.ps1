# Test: --create-file switch forces file creation without an extension.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_create_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# --- Test ---
Write-Host "Running test: ir create --create-file my_file"
& $Executable create --create-file "my_file"

# --- Verification ---
$Result = 1
if (Test-Path "my_file" -PathType Leaf) {
    Write-Host "✅ PASS: File 'my_file' was created successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File 'my_file' was not created."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
