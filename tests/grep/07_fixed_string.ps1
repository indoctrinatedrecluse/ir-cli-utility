# Test: grep with fixed strings (-F flag).

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_07"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("test.file", "test*file", "test[file]", "test.ext")

# --- Test ---
Write-Host "Running test: ir grep -F 'test.*' sample.txt"
$Output = & $Executable grep -F "test.*" $File | Out-String

# --- Verification ---
$Result = 1
if (($Output -match "test\*file") -and -not ($Output -match "test\.file" -and $Output -split "`n" | Where-Object { $_ -match "^test\.file$" })) {
    Write-Host "PASS: grep -F treated pattern as literal string."
    $Result = 0
} else {
    Write-Host "FAIL: grep -F should match literal 'test.*' only"
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

