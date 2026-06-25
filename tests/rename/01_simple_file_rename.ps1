# Test: Simple file rename

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# Create a temporary directory for the test
$TestDir = "temp_test_rename_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create a file to rename
New-Item -ItemType File -Name "original.txt" | Out-Null

# --- Test ---
Write-Host "Running test: ir rename original.txt renamed.txt"
& $Executable rename "original.txt" "renamed.txt"

# --- Verification ---
$Result = 1
if ((Test-Path "renamed.txt") -and -not (Test-Path "original.txt")) {
    Write-Host "✅ PASS: 'renamed.txt' exists and 'original.txt' does not."
    $Result = 0
} else {
    Write-Host "❌ FAIL: 'renamed.txt' was not created or 'original.txt' was not removed."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
