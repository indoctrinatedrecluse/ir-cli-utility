# Test: grep with stdin piping.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test ---
Write-Host "Running test: echo content | ir grep 'test'"
$Input = @("this is a test", "no match", "another test", "nope")
$Output = ($Input | & $Executable grep "test") | Out-String

# --- Verification ---
$Result = 1
$LineCount = ($Output -split "`n" | Where-Object { $_ -match "test" } | Measure-Object).Count
if ($LineCount -eq 2) {
    Write-Host "PASS: grep via stdin found 2 lines matching 'test'."
    $Result = 0
} else {
    Write-Host "FAIL: grep via stdin should have found 2 lines, found $LineCount"
    Write-Host "Output was:"
    Write-Host $Output
}

exit $Result

