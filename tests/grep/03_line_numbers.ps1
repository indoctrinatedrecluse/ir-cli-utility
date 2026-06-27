# Test: grep with line numbers (-n flag).

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_03"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("line one", "line two", "line three", "line four")

# --- Test ---
Write-Host "Running test: ir grep -n 'three' sample.txt"
$Output = & $Executable grep -n "three" $File | Out-String

# --- Verification ---
$Result = 1
if ($Output -match ":3:" -or $Output -match "3\s+") {
    Write-Host "PASS: grep -n displayed line number 3."
    $Result = 0
} else {
    Write-Host "FAIL: grep -n should have displayed line number."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

