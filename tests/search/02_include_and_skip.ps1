# Test: search include filters and default skipped extensions.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_search_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
Set-Content -Path "$TestDir\keep.rs" -Value "needle in rust"
Set-Content -Path "$TestDir\skip.txt" -Value "needle in text"
Set-Content -Path "$TestDir\archive.zip" -Value "needle in archive"

# --- Test ---
Write-Host "Running test: ir search needle temp_test_search_02 --include rs"
$IncludeOutput = & $Executable search needle $TestDir --include rs | Out-String
$DefaultOutput = & $Executable search needle $TestDir | Out-String

# --- Verification ---
$Result = 1
if (($IncludeOutput -like "*keep.rs*") -and ($IncludeOutput -notlike "*skip.txt*") -and ($DefaultOutput -notlike "*archive.zip*")) {
    Write-Host "PASS: search respected include filters and skipped archive extension."
    $Result = 0
} else {
    Write-Host "FAIL: search did not filter files as expected."
    Write-Host "Include output was:"
    Write-Host $IncludeOutput
    Write-Host "Default output was:"
    Write-Host $DefaultOutput
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
