# Test: Simple whoami
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

Write-Host "Running test: ir whoami"
$output = & $Executable whoami
Write-Host "Output: $output"

$Result = 1
if ($output -match "[a-zA-Z0-9]") {
    Write-Host "✅ PASS: 'ir whoami' returned a non-empty identity."
    $Result = 0
} else {
    Write-Host "❌ FAIL: 'ir whoami' returned empty identity."
}

exit $Result
