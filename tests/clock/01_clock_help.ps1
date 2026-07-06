# Test: clock command help and invalid parameters

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"

$Passed = $true
function Assert-Contains($Actual, $Expected, $Msg) {
    if ($Actual.Contains($Expected)) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected to contain '$Expected')"
        $global:Passed = $false
    }
}

# --- 1. ir help clock ---
Write-Host "Testing 'ir help clock'..."
$HelpOut = & $Executable help clock | Out-String
Assert-Contains $HelpOut "ir-clock" "Header matches clock help"
Assert-Contains $HelpOut "USAGE:" "Usage section matches"
Assert-Contains $HelpOut "Tab / C" "Hotkeys section matches"

# --- 2. Invalid modes ---
Write-Host "Testing invalid mode rejection..."
$ErrOut = & $Executable clock --mode invalid_mode 2>&1 | Out-String
Assert-Contains $ErrOut "Error: Invalid mode 'invalid_mode'" "Invalid mode switch error message printed"

# --- 3. Invalid timer duration format ---
Write-Host "Testing invalid timer duration format rejection..."
$TimerErrOut = & $Executable clock --timer abc 2>&1 | Out-String
Assert-Contains $TimerErrOut "Error: Invalid timer duration format 'abc'" "Invalid timer duration error message printed"

if ($Passed) {
    Write-Host "`nALL CLOCK HELP TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME CLOCK HELP TESTS FAILED"
    exit 1
}
