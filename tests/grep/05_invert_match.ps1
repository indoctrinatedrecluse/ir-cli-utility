# Test: grep with invert match (-v flag).

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_05"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File = Join-Path $TestDir "sample.txt"
Set-Content -Path $File -Value @("apple", "banana", "apricot", "berry", "avocado")

# --- Test ---
Write-Host "Running test: ir grep -v 'a' sample.txt"
$Output = & $Executable grep -v "a" $File | Out-String

# --- Verification ---
$Result = 1
# Only 'berry' does NOT contain 'a'. All others (apple, banana, apricot, avocado) contain 'a'
if (($Output -match "berry") -and -not ($Output -match "apple") -and -not ($Output -match "banana")) {
    Write-Host "PASS: grep -v found only 'berry' (the line without 'a')."
    $Result = 0
} else {
    Write-Host "FAIL: grep -v should have found only 'berry'"
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

