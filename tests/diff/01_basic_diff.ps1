# Test: diff reports changed lines.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_diff_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
Set-Content -Path "$TestDir\left.txt" -Value @("same", "old", "done")
Set-Content -Path "$TestDir\right.txt" -Value @("same", "new", "done")

# --- Test ---
Write-Host "Running test: ir diff left.txt right.txt"
$Output = & $Executable diff "$TestDir\left.txt" "$TestDir\right.txt" | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*2c2*") -and ($Output -like "*< old*") -and ($Output -like "*> new*")) {
    Write-Host "PASS: diff reported the changed line."
    $Result = 0
} else {
    Write-Host "FAIL: diff output did not contain the expected change."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
