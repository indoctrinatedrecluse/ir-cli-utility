# Test: find with root paths supplied through stdin.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_find_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path "$TestDir\docs" | Out-Null
New-Item -ItemType File -Path "$TestDir\docs\README.TXT" | Out-Null
New-Item -ItemType File -Path "$TestDir\notes.md" | Out-Null

# --- Test ---
Write-Host "Running test: path | ir find -iname '*readme*'"
$Output = @("$TestDir\docs") | & $Executable find -iname "*readme*" | Out-String

# --- Verification ---
$Result = 1
if (($Output -like "*README.TXT*") -and ($Output -notlike "*notes.md*")) {
    Write-Host "PASS: find searched piped root paths."
    $Result = 0
} else {
    Write-Host "FAIL: find did not search piped root paths as expected."
    Write-Host "Output was:"
    Write-Host $Output
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

exit $Result
