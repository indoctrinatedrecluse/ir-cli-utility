# Test: ir dua — verify help routing, ncdu alias, and invalid switch handling.
# Run from project root: .\tests\dua\01_dua_help.ps1

Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$Result = 0

# 1. Help routing
Write-Host "Testing 'ir help dua'..."
$out = & $Executable help dua 2>&1 | Out-String
if ($out -match "ir-dua" -and $out -match "PATH") {
    Write-Host "✅ PASS: 'ir help dua' returned correct help text."
} else {
    Write-Host "❌ FAIL: 'ir help dua' did not return expected output: $out"
    $Result = 1
}

# 2. Alias 'ncdu' maps to dua help
Write-Host "Testing 'ir help ncdu' alias..."
$out2 = & $Executable help ncdu 2>&1 | Out-String
if ($out2 -match "ir-dua") {
    Write-Host "✅ PASS: 'ir help ncdu' alias correctly routed to dua help."
} else {
    Write-Host "❌ FAIL: 'ir help ncdu' alias did not route correctly: $out2"
    $Result = 1
}

# 3. Invalid switch detected
Write-Host "Testing 'ir dua' with invalid switch..."
$err = & $Executable dua --badswitch 2>&1 | Out-String
if ($err -match "Unknown switch" -or $err -match "ir-dua") {
    Write-Host "✅ PASS: 'ir dua' with invalid switch produced error/help."
} else {
    Write-Host "❌ FAIL: 'ir dua' invalid switch not caught: $err"
    $Result = 1
}

exit $Result
