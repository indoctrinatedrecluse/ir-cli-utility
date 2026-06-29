# Test: recursive search finds matching file contents.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_search_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path "$TestDir\src\nested" | Out-Null
Set-Content -Path "$TestDir\src\main.rs" -Value @("fn main() {}", "TODO: wire feature")
Set-Content -Path "$TestDir\src\nested\notes.txt" -Value "nothing here"

# --- Test ---
Write-Host "Running test: ir search TODO temp_test_search_01"
$Output = & $Executable search TODO $TestDir | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*main.rs:2:TODO: wire feature*") -and ($Output -notlike "*notes.txt*")) {
    Write-Host "PASS: search found the recursive content match."
    $Result = 0
} else {
    Write-Host "FAIL: search output did not match expected recursive search behavior."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
