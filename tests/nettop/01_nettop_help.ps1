# Test: ir nettop — verify help routing and alias.
# Run from project root: .\tests\nettop\01_nettop_help.ps1

Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$Result = 0

# 1. Help routing
Write-Host "Testing 'ir help nettop'..."
$out = & $Executable help nettop 2>&1 | Out-String
if ($out -match "ir-nettop" -and $out -match "delay") {
    Write-Host "✅ PASS: 'ir help nettop' returned correct help text."
} else {
    Write-Host "❌ FAIL: 'ir help nettop' did not return expected output: $out"
    $Result = 1
}

# 2. Alias 'ntop' maps to nettop help
Write-Host "Testing 'ir help ntop' alias..."
$out2 = & $Executable help ntop 2>&1 | Out-String
if ($out2 -match "ir-nettop") {
    Write-Host "✅ PASS: 'ir help ntop' alias correctly routed to nettop help."
} else {
    Write-Host "❌ FAIL: 'ir help ntop' alias did not route correctly: $out2"
    $Result = 1
}

# 3. Invalid switch detected
Write-Host "Testing 'ir nettop' with invalid switch..."
$err = & $Executable nettop --badswitch 2>&1 | Out-String
if ($err -match "Unknown switch" -or $err -match "ir-nettop") {
    Write-Host "✅ PASS: 'ir nettop' with invalid switch produced error/help."
} else {
    Write-Host "❌ FAIL: 'ir nettop' invalid switch not caught: $err"
    $Result = 1
}

exit $Result
