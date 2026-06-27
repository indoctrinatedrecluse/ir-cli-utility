# Test: grep on multiple files.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_grep_08"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null
$File1 = Join-Path $TestDir "file1.txt"
$File2 = Join-Path $TestDir "file2.txt"
Set-Content -Path $File1 -Value @("match here", "no match")
Set-Content -Path $File2 -Value @("another match", "still no", "match here too")

# --- Test ---
Write-Host "Running test: ir grep 'match' file1.txt file2.txt"
$Output = & $Executable grep "match" $File1 $File2 | Out-String

# --- Verification ---
$Result = 1
$MatchCount = ($Output -split "`n" | Where-Object { $_ -match "match" } | Measure-Object).Count
if ($MatchCount -ge 3) {
    Write-Host "PASS: grep found matches across multiple files."
    $Result = 0
} else {
    Write-Host "FAIL: grep should have found at least 3 matches, found $MatchCount"
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result

