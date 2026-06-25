# Test: Interactive rename with 'y' confirmation.

# --- Setup ---
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_rename_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir
New-Item -ItemType File -Name "interactive.txt" | Out-Null

# --- Test ---
Write-Host "Running test: 'y' | ir rename -i interactive.txt confirmed.txt"
# Pipe 'y' and a newline into the command.
"y" | & $Executable rename -i "interactive.txt" "confirmed.txt"

# --- Verification ---
$Result = 1
if ((Test-Path "confirmed.txt") -and -not (Test-Path "interactive.txt")) {
    Write-Host "✅ PASS: File was successfully renamed after confirmation."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File was not renamed correctly."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
