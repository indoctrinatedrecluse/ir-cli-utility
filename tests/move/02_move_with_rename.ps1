# Test: Move a single file with the --rename switch.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_move_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType File -Name "source.log" | Out-Null
New-Item -ItemType Directory -Name "logs_archive" | Out-Null

# --- Test ---
Write-Host "Running test: ir move source.log logs_archive --rename archive.log"
& $Executable move "source.log" "logs_archive" --rename "archive.log"

# --- Verification ---
$Result = 1
if ((Test-Path "logs_archive/archive.log") -and -not (Test-Path "source.log")) {
    Write-Host "✅ PASS: File was moved and renamed successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Renamed file not found or source file still exists."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
