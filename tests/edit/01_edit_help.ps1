# Test: ir edit — help routing, alias, error handling for missing filename and invalid switch.
# Run from project root: .\tests\edit\01_edit_help.ps1
#
# NOTE: Interactive editing (keystroke simulation) cannot be automated in a non-TTY
# environment. These tests cover all automatable surface: routing, aliases, and
# CLI error guards. Actual editing is validated manually via 'ir edit <file>'.

Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$Result = 0

# 1. Help text is correct
Write-Host "Testing 'ir help edit'..."
$out = & $Executable help edit 2>&1 | Out-String
if ($out -match "ir-edit" -and $out -match "Ctrl\+S" -and $out -match "Ctrl\+Q" -and $out -match "Arrow keys") {
    Write-Host "✅ PASS: 'ir help edit' returned correct help text."
} else {
    Write-Host "❌ FAIL: 'ir help edit' unexpected output: $out"
    $Result = 1
}

# 2. Alias 'ed' routes to edit help
Write-Host "Testing 'ir help ed' alias..."
$out2 = & $Executable help ed 2>&1 | Out-String
if ($out2 -match "ir-edit") {
    Write-Host "✅ PASS: 'ir help ed' alias correctly routed to edit help."
} else {
    Write-Host "❌ FAIL: 'ir help ed' alias did not route correctly: $out2"
    $Result = 1
}

# 3. Calling 'ir edit' without a filename produces an error and shows help
Write-Host "Testing 'ir edit' with no filename..."
$err = & $Executable edit 2>&1 | Out-String
if ($err -match "requires a filename" -or $err -match "ir-edit") {
    Write-Host "✅ PASS: 'ir edit' (no filename) correctly showed error/help."
} else {
    Write-Host "❌ FAIL: 'ir edit' (no filename) did not error correctly: $err"
    $Result = 1
}

# 4. Unknown switch produces an error and shows help
Write-Host "Testing 'ir edit' with an unknown switch..."
$err2 = & $Executable edit --badswitch 2>&1 | Out-String
if ($err2 -match "Unknown switch" -or $err2 -match "ir-edit") {
    Write-Host "✅ PASS: 'ir edit --badswitch' correctly showed error/help."
} else {
    Write-Host "❌ FAIL: 'ir edit --badswitch' did not error correctly: $err2"
    $Result = 1
}

# 5. Passing a directory path as filename should error (is a directory, not a file)
Write-Host "Testing 'ir edit' on a directory path..."
$dirErr = & $Executable edit "src" 2>&1 | Out-String
if ($dirErr -match "directory" -or $dirErr -match "is a directory") {
    Write-Host "✅ PASS: 'ir edit src' correctly rejected a directory."
} else {
    Write-Host "❌ FAIL: 'ir edit src' did not reject directory: $dirErr"
    $Result = 1
}

exit $Result
