# Test: Basic cat functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_cat_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("alpha", "beta", "gamma")

# --- Test ---
Write-Host "Running test: ir cat sample.txt"
$Output = & $Executable cat $File | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*alpha*") -and ($Output -like "*beta*") -and ($Output -like "*gamma*")) {
    Write-Host "PASS: 'cat' printed the file contents."
    $Result = 0
} else {
    Write-Host "FAIL: 'cat' output was missing expected content."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
