# Test: grep with count (-c flag).

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_04"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("todo: fix bug", "done: review code", "todo: add tests", "todo: update docs", "done: deploy")

# --- Test ---
Write-Host "Running test: ir grep -c 'todo' sample.txt"
$Output = & $Executable grep -c "todo" $File | Out-String

# --- Verification ---
$Result = 1
if ($Output -match "3") {
    Write-Host "PASS: grep -c returned count of 3."
    $Result = 0
} else {
    Write-Host "FAIL: grep -c should have returned 3, got: $Output"
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

