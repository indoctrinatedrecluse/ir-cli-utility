# Test: ir browse — verify help routing, fm alias, and invalid switch handling.
# Run from project root: .\tests\browse\01_browse_help.ps1

Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$Result = 0

# 1. Help routing
Write-Host "Testing 'ir help browse'..."
$out = & $Executable help browse 2>&1 | Out-String
if ($out -match "ir-browse" -and $out -match "PATH" -and $out -match "Copy" -and $out -match "Delete") {
    Write-Host "✅ PASS: 'ir help browse' returned correct help text."
} else {
    Write-Host "❌ FAIL: 'ir help browse' did not return expected output: $out"
    $Result = 1
}

# 2. Alias 'fm' maps to browse help
Write-Host "Testing 'ir help fm' alias..."
$out2 = & $Executable help fm 2>&1 | Out-String
if ($out2 -match "ir-browse") {
    Write-Host "✅ PASS: 'ir help fm' alias correctly routed to browse help."
} else {
    Write-Host "❌ FAIL: 'ir help fm' alias did not route correctly: $out2"
    $Result = 1
}

# 3. Invalid switch detected
Write-Host "Testing 'ir browse' with invalid switch..."
$err = & $Executable browse --badswitch 2>&1 | Out-String
if ($err -match "Unknown switch" -or $err -match "ir-browse") {
    Write-Host "✅ PASS: 'ir browse' with invalid switch produced error/help."
} else {
    Write-Host "❌ FAIL: 'ir browse' invalid switch not caught: $err"
    $Result = 1
}

exit $Result
