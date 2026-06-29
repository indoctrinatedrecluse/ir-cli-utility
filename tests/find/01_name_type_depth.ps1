# Test: find with name, type, and depth expressions.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_find_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path "$TestDir\src\nested" | Out-Null
New-Item -ItemType File -Path "$TestDir\src\main.rs" | Out-Null
New-Item -ItemType File -Path "$TestDir\src\nested\lib.rs" | Out-Null
New-Item -ItemType File -Path "$TestDir\README.md" | Out-Null

# --- Test ---
Write-Host "Running test: ir find temp_test_find_01 -name '*.rs' -type f -maxdepth 2"
$Output = & $Executable find $TestDir -name "*.rs" -type f -maxdepth 2 | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*main.rs*") -and ($Output -notlike "*lib.rs*") -and ($Output -notlike "*README.md*")) {
    Write-Host "PASS: find matched the expected Rust file within max depth."
    $Result = 0
} else {
    Write-Host "FAIL: find output did not match expected name/type/depth filtering."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
