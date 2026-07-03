# Test: Simple df
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

Write-Host "Running test: ir df"
$output = & $Executable df
Write-Host $output

$Result = 1
if ($output -match "Device" -and $output -match "Size") {
    Write-Host "✅ PASS: 'ir df' output header verified."
    $Result = 0
} else {
    Write-Host "❌ FAIL: 'ir df' output headers missing."
}

exit $Result
