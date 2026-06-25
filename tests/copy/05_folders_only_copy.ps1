# Test: Copy with -l (folders only).

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_copy_05"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType Directory -Path "source_dir\subdir" | Out-Null
New-Item -ItemType File -Path "source_dir\file1.txt" | Out-Null
New-Item -ItemType Directory -Name "dest_dir" | Out-Null

# --- Test ---
Write-Host "Running test: ir copy -l source_dir dest_dir"
& $Executable copy -l "source_dir" "dest_dir"

# --- Verification ---
$Result = 1
if ((Test-Path "dest_dir\source_dir\subdir") -and -not (Test-Path "dest_dir\source_dir\file1.txt")) {
    Write-Host "✅ PASS: Only the subdirectory was copied, not the file."
    $Result = 0
} else {
    Write-Host "❌ FAIL: The file was copied or the subdirectory was not."
    Get-ChildItem -Recurse -Path "dest_dir"
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
