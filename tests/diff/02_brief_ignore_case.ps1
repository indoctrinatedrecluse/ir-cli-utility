# Test: diff brief mode with ignore-case.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_diff_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
Set-Content -Path "$TestDir\left.txt" -Value "Hello"
Set-Content -Path "$TestDir\right.txt" -Value "hello"

# --- Test ---
Write-Host "Running test: ir diff -qi left.txt right.txt"
$Output = & $Executable diff -q -i "$TestDir\left.txt" "$TestDir\right.txt" | Out-String

# --- Verification ---
$Result = 1
if ([string]::IsNullOrWhiteSpace($Output)) {
    Write-Host "PASS: diff ignored case differences."
    $Result = 0
} else {
    Write-Host "FAIL: diff should not have reported case-only differences."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
