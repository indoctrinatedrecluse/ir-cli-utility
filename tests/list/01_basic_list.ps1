# Test: Basic list functionality.

# --- Setup ---
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_list_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir
New-Item -ItemType File -Name "file1.txt" | Out-Null
New-Item -ItemType Directory -Name "folder1" | Out-Null

# --- Test ---
Write-Host "Running test: ir list"
$Output = & $Executable list | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*file1.txt*") -and ($Output -like "*folder1*")) {
    Write-Host "✅ PASS: 'list' command output contains the created file and folder."
    $Result = 0
} else {
    Write-Host "❌ FAIL: 'list' command output is missing expected items."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
