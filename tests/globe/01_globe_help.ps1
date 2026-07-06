# Test: globe help and parameters

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

# --- 1. ir help globe ---
Write-Host "Testing 'ir help globe'..."
$HelpOut = & $Executable help globe | Out-String
Assert-Contains $HelpOut "ir-globe" "Header matches globe help"
Assert-Contains $HelpOut "USAGE:" "Usage section matches"
Assert-Contains $HelpOut "Tab / M" "Hotkeys section matches"

# --- 2. Invalid modes ---
Write-Host "Testing invalid mode rejection..."
$ErrOut = & $Executable globe --mode invalid_mode 2>&1 | Out-String
Assert-Contains $ErrOut "Error: Invalid mode 'invalid_mode'" "Invalid mode switch error message printed"

# --- 3. Invalid center coordinates ---
Write-Host "Testing invalid center coordinate format..."
$CenterErrOut = & $Executable globe --center abc 2>&1 | Out-String
Assert-Contains $CenterErrOut "Error: --center must be formatted as 'lat,lon'" "Invalid center format error message printed"

# --- 4. Out of range coordinates ---
Write-Host "Testing out of range center coordinates..."
$RangeErrOut = & $Executable globe --center 95.0,200.0 2>&1 | Out-String
Assert-Contains $RangeErrOut "Error: Coordinates must be in range -90..90" "Coordinates out of range error message printed"

if ($Passed) {
    Write-Host "`nALL GLOBE HELP TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME GLOBE HELP TESTS FAILED"
    exit 1
}
