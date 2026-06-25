# Test: Copy a single file with the --rename switch.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_copy_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

New-Item -ItemType File -Name "source.log" | Out-Null
New-Item -ItemType Directory -Name "logs_backup" | Out-Null

# --- Test ---
Write-Host "Running test: ir copy source.log logs_backup --rename backup.log"
& $Executable copy "source.log" "logs_backup" --rename "backup.log"

# --- Verification ---
$Result = 1
if (Test-Path "logs_backup/backup.log") {
    Write-Host "✅ PASS: File was copied and renamed successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Renamed file not found in the destination directory."
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
