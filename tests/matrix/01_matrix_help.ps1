# Test: matrix parameter validation and help display

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

# 1. Test invalid mode parameter error validation
Write-Host "Testing invalid mode validation..."
$ErrOut = & $Executable matrix -m invalid 2>&1 | Out-String
Assert-Contains $ErrOut "Error: Unsupported mode 'invalid'" "Validation catches unsupported mode argument"
Assert-Contains $ErrOut "ir-matrix" "Validation prints help page on error"

# 2. Test invalid FPS parameter validation
Write-Host "Testing invalid FPS validation..."
$ErrOut2 = & $Executable matrix -f 100 2>&1 | Out-String
Assert-Contains $ErrOut2 "Error: Invalid FPS '100'. Must be between 1 and 60." "Validation catches invalid FPS boundaries"

# 3. Test help command output details
Write-Host "Testing help command matches..."
$HelpOut = & $Executable help matrix | Out-String
Assert-Contains $HelpOut "Screensaver effect mode: 'matrix' or 'fire'" "Help prints mode descriptions"
Assert-Contains $HelpOut "Cycle through color schemes" "Help prints interactive controls keys"

if ($Passed) {
    Write-Host "`nALL MATRIX TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME MATRIX TESTS FAILED"
    exit 1
}
