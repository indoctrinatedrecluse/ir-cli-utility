# Test: Default behavior creates a file when an extension is present.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_create_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# --- Test ---
Write-Host "Running test: ir create new_file.txt"
& $Executable create "new_file.txt"

# --- Verification ---
$Result = 1
if (Test-Path "new_file.txt" -PathType Leaf) {
    Write-Host "✅ PASS: File 'new_file.txt' was created successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File 'new_file.txt' was not created."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
