# Test: cat head, tail, and range selectors.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_cat_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("one", "two", "three", "four")

# --- Test ---
Write-Host "Running test: ir cat selectors"
$Head = & $Executable cat --head 2 $File | Out-String
$Tail = & $Executable cat --tail 2 $File | Out-String
$Range = & $Executable cat --range 2:3 $File | Out-String

# --- Verification ---
$Result = 1
if (($Head -like "*one*") -and ($Head -like "*two*") -and ($Head -notlike "*three*") -and
    ($Tail -like "*three*") -and ($Tail -like "*four*") -and ($Tail -notlike "*two*") -and
    ($Range -like "*two*") -and ($Range -like "*three*") -and ($Range -notlike "*one*") -and ($Range -notlike "*four*")) {
    Write-Host "PASS: 'cat' selectors printed the expected lines."
    $Result = 0
} else {
    Write-Host "FAIL: selector output was not as expected."
    Write-Host "Head was:"
    Write-Host $Head
    Write-Host "Tail was:"
    Write-Host $Tail
    Write-Host "Range was:"
    Write-Host $Range
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
