# Test: ir watch — verify help routing and invalid switch handling.
# Run from project root: .\tests\watch\01_watch_help.ps1

Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$Result = 0

# 1. Help routing
Write-Host "Testing 'ir help watch'..."
$out = & $Executable help watch 2>&1 | Out-String
if ($out -match "ir-watch" -and $out -match "interval" -and $out -match "diff") {
    Write-Host "✅ PASS: 'ir help watch' returned correct help text."
} else {
    Write-Host "❌ FAIL: 'ir help watch' did not return expected output: $out"
    $Result = 1
}

# 2. Invalid switch detected
Write-Host "Testing 'ir watch' with invalid switch..."
$err = & $Executable watch --unknown 2>&1 | Out-String
if ($err -match "Unknown switch" -or $err -match "ir-watch") {
    Write-Host "✅ PASS: 'ir watch' with invalid switch produced error/help."
} else {
    Write-Host "❌ FAIL: 'ir watch' invalid switch not caught: $err"
    $Result = 1
}

exit $Result
