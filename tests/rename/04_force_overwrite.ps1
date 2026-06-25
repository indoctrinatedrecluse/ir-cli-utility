# Test: Force rename overwrites an existing destination file.

# --- Setup ---
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_rename_04"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir
New-Item -ItemType File -Name "source_file" | Out-Null
Set-Content -Path "destination_file" -Value "destination content"

# --- Test ---
Write-Host "Running test: ir rename -f source_file destination_file"
& $Executable rename -f "source_file" "destination_file"

# --- Verification ---
$Result = 1
$DestFile = Get-Item "destination_file"
# The destination should now exist, but be empty (like the source was).
if ((Test-Path "destination_file") -and -not (Test-Path "source_file") -and ($DestFile.Length -eq 0)) {
    Write-Host "✅ PASS: Destination was overwritten successfully."
    $Result = 0
} else {
    Write-Host "❌ FAIL: Destination was not overwritten correctly."
    Get-ChildItem | ForEach-Object { Write-Host $_.Name, $_.Length }
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
