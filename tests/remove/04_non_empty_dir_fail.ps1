# Test: Default behavior fails to remove a non-empty directory without -y or -f.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_remove_04"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType Directory -Path "dir_to_keep\subdir" | Out-Null

# --- Test ---
Write-Host "Running test: ir remove dir_to_keep (with 'n' piped in)"
# Pipe 'n' to the confirmation prompt.
$Output = "n" | & $Executable remove "dir_to_keep" | Out-String

# --- Verification ---
$Result = 1
if ((Test-Path "dir_to_keep") -and ($Output -like "*Operation cancelled.*")) {
    Write-Host "✅ PASS: Operation was cancelled as expected for non-empty directory."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Directory was removed or cancellation message was not shown."
    Write-Host "Output was: $Output"
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
