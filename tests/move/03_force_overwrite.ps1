# Test: Force move overwrites an existing destination file.

# --- Setup ---
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_move_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

Set-Content -Path "source_file" -Value "source content"
New-Item -ItemType Directory -Name "dest_dir" | Out-Null
Set-Content -Path "dest_dir\source_file" -Value "destination content"

# --- Test ---
Write-Host "Running test: ir move --force source_file dest_dir"
& $Executable move --force "source_file" "dest_dir"

# --- Verification ---
$Result = 1
$DestContent = Get-Content "dest_dir\source_file"
if ((Test-Path "dest_dir\source_file") -and -not (Test-Path "source_file") -and ($DestContent -eq "source content")) {
    Write-Host "✅ PASS: Destination was overwritten successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Destination was not overwritten correctly or source was not removed."
    Get-ChildItem -Recurse | ForEach-Object { Write-Host $_.Name, $_.Length }
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
