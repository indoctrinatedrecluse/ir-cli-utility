# Test: Moving a file to the Recycle Bin with -t.
# NOTE: This test is difficult to verify automatically in a script without complex API calls.
# It primarily serves to execute the code path. Manual verification by checking
# the Recycle Bin may be needed if issues are suspected.

# --- Setup ---
Write-Host "Building..."
cargo build --quiet
$Executable = "..\target\debug\ir.exe"
$TestDir = "temp_test_remove_05"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

$TrashFileName = "file_to_trash.txt"
New-Item -ItemType File -Name $TrashFileName | Out-Null

# --- Test ---
Write-Host "Running test: ir remove -t $TrashFileName"
# The -y is added to skip any potential prompts for this basic test
& $Executable remove -ty $TrashFileName

# --- Verification ---
$Result = 1
if (-not (Test-Path $TrashFileName)) {
    Write-Host "✅ PASS: File was removed from the directory (presumably moved to trash)."
    $Result = 0
} else {
    Write-Host "❌ FAIL: File still exists after trash operation."
}

# --- Teardown ---
# No teardown needed for the file itself, as it's in the Recycle Bin.
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
