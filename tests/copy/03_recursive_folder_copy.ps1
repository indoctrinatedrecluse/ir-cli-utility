# Test: Default recursive copy of a directory.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_copy_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType Directory -Path "source_dir\subdir" | Out-Null
New-Item -ItemType File -Path "source_dir\file1.txt" | Out-Null
New-Item -ItemType File -Path "source_dir\subdir\file2.txt" | Out-Null
New-Item -ItemType Directory -Name "dest_dir" | Out-Null

# --- Test ---
Write-Host "Running test: ir copy source_dir dest_dir"
& $Executable copy "source_dir" "dest_dir"

# --- Verification ---
$Result = 1
if ((Test-Path "dest_dir\source_dir\subdir") -and (Test-Path "dest_dir\source_dir\file1.txt") -and (Test-Path "dest_dir\source_dir\subdir\file2.txt")) {
    Write-Host "✅ PASS: Directory was copied recursively."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Recursive copy did not produce the expected structure."
    Get-ChildItem -Recurse -Path "dest_dir"
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
