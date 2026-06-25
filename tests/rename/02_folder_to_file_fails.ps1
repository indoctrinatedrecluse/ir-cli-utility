# Test: Renaming a folder to a file with an extension should fail.

# --- Setup ---
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_rename_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir
New-Item -ItemType Directory -Name "my_folder" | Out-Null

# --- Test ---
Write-Host "Running test: ir rename my_folder invalid.txt"
# We expect this command to fail. We use a try/catch block to handle the non-zero exit code.
$Output = ""
try {
    # Redirect stderr (2) to stdout (1) to capture the error message
    & $Executable rename "my_folder" "invalid.txt" 2>&1 | Out-String
} catch {
    $Output = $_.Exception.Message
}

# --- Verification ---
$Result = 1
if ($Output -like "*Error: Cannot rename a folder*") {
    Write-Host "✅ PASS: Command failed with the correct error message."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Command did not produce the expected error."
    Write-Host "Output was: $Output"
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
