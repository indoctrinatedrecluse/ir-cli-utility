# Test: -p switch creates parent directories.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_create_04"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# --- Test ---
Write-Host "Running test: ir create -p new\nested\directory"
& $Executable create -p "new\nested\directory"

# --- Verification ---
$Result = 1
if (Test-Path "new\nested\directory" -PathType Container) {
    Write-Host "✅ PASS: Nested directory structure was created successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Nested directory was not created."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
