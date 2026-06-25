# Test: Interactive remove where the user cancels.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_remove_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir
New-Item -ItemType File -Name "file_to_keep.txt" | Out-Null

# --- Test ---
Write-Host "Running test: 'n' | ir remove -i file_to_keep.txt"
# Pipe 'n' and a newline into the command to simulate user cancellation.
$Output = "n" | & $Executable remove -i "file_to_keep.txt" | Out-String

# --- Verification ---
$Result = 1
if ((Test-Path "file_to_keep.txt") -and ($Output -like "*Operation cancelled.*")) {
    Write-Host "✅ PASS: File was not removed and cancellation message was shown."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File was removed or cancellation message was not shown."
    Write-Host "Output was: $Output"
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
