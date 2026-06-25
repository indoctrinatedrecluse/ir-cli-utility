# Test: cat line numbering.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_cat_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("alpha", "beta")

# --- Test ---
Write-Host "Running test: ir cat -n sample.txt"
$Output = & $Executable cat -n $File | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*1*alpha*") -and ($Output -like "*2*beta*")) {
    Write-Host "PASS: 'cat -n' printed line numbers."
    $Result = 0
} else {
    Write-Host "FAIL: 'cat -n' output was missing expected line numbers."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
