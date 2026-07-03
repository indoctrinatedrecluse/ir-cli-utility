# Test: Simple sockets
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

Write-Host "Running test: ir sockets -a"
$output = & $Executable sockets -a
Write-Host $output

$Result = 1
if ($output -match "Proto" -and $output -match "Local Address") {
    Write-Host "✅ PASS: 'ir sockets' output header verified."
    $Result = 0
} else {
    Write-Host "❌ FAIL: 'ir sockets' output headers missing."
}

exit $Result
