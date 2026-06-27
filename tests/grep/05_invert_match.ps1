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
$Lines = $Output -split "`n" | Where-Object { $_ -match "\S" }
if (($Lines.Count -eq 2) -and ($Output -match "banana") -and ($Output -match "berry")) {
    Write-Host "PASS: grep -v found 2 lines without 'a'."
    $Result = 0
} else {
    Write-Host "FAIL: grep -v should have found 2 lines (banana, berry)"
    Write-Host "Found $($Lines.Count) lines:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

