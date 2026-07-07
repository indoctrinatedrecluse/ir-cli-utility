# Test: life help and parameters

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

# --- 1. ir help life ---
Write-Host "Testing 'ir help life'..."
$HelpOut = & $Executable help life | Out-String
Assert-Contains $HelpOut "ir-life" "Header matches life help"
Assert-Contains $HelpOut "USAGE:" "Usage section matches"
Assert-Contains $HelpOut "Glider" "Hotkeys section matches"

# --- 2. Invalid FPS ---
Write-Host "Testing invalid FPS rejection..."
$ErrOut = & $Executable life --fps 45 2>&1 | Out-String
Assert-Contains $ErrOut "Error: --fps must be a valid integer between 1 and 30" "Invalid FPS range error message printed"

# --- 3. Invalid Presets ---
Write-Host "Testing invalid preset pattern rejection..."
$PresetErrOut = & $Executable life --preset invalid_preset 2>&1 | Out-String
Assert-Contains $PresetErrOut "Error: Invalid preset 'invalid_preset'" "Invalid preset name error message printed"

if ($Passed) {
    Write-Host "`nALL LIFE HELP TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME LIFE HELP TESTS FAILED"
    exit 1
}
