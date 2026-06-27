# Test: Case-insensitive grep with -i flag.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("ERROR: critical failure", "Warning: low memory", "Error: disk space", "info: normal")

# --- Test ---
Write-Host "Running test: ir grep -i 'error' sample.txt"
$Output = & $Executable grep -i "error" $File | Out-String

# --- Verification ---
$Result = 1
$LineCount = ($Output -split "`n" | Where-Object { $_ -match "ERROR|Error" } | Measure-Object).Count
if ($LineCount -eq 2) {
    Write-Host "PASS: grep -i found 2 lines matching 'error' (case-insensitive)."
    $Result = 0
} else {
    Write-Host "FAIL: grep -i should have found 2 lines, found $LineCount"
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

