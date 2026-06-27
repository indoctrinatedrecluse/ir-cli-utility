# Test: Basic grep functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("error: something failed", "warning: be careful", "error: another issue", "info: all good")

# --- Test ---
Write-Host "Running test: ir grep 'error' sample.txt"
$Output = & $Executable grep "error" $File | Out-String

# --- Verification ---
$Result = 1
$LineCount = ($Output -split "`n" | Where-Object { $_ -match "error" } | Measure-Object).Count
if ($LineCount -eq 2) {
    Write-Host "PASS: grep found 2 lines matching 'error'."
    $Result = 0
} else {
    Write-Host "FAIL: grep should have found 2 lines, found $LineCount"
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

