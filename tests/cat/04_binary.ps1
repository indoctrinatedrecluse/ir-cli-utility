# Test: cat binary preview.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_cat_04"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.bin"
[System.IO.File]::WriteAllBytes($File, [byte[]](0x41, 0x42, 0x00, 0x43))

# --- Test ---
Write-Host "Running test: ir cat --binary sample.bin"
$Output = & $Executable cat --binary $File | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*00000000*") -and ($Output -like "*41 42 00 43*") -and ($Output -like "*AB.C*")) {
    Write-Host "PASS: 'cat --binary' printed a hexadecimal preview."
    $Result = 0
} else {
    Write-Host "FAIL: binary preview output was not as expected."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
