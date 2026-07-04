# Test: Simple chmod on Windows
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$TestFile = "temp_chmod_test.txt"

# Clean up
if (Test-Path $TestFile) { Remove-Item $TestFile -Force }

"initial content" | Out-File -FilePath $TestFile -Encoding ascii -NoNewline

$Result = 0

Write-Host "Applying chmod 444 (setting read-only)..."
& $Executable chmod 444 $TestFile

$meta1 = Get-Item $TestFile
if ($meta1.IsReadOnly -eq $true) {
    Write-Host "✅ PASS: File is read-only after chmod 444."
} else {
    Write-Host "❌ FAIL: File is not read-only after chmod 444."
    $Result = 1
}

Write-Host "Applying chmod 644 (removing read-only)..."
& $Executable chmod 644 $TestFile

$meta2 = Get-Item $TestFile
if ($meta2.IsReadOnly -eq $false) {
    Write-Host "✅ PASS: File is writeable (not read-only) after chmod 644."
} else {
    Write-Host "❌ FAIL: File is still read-only after chmod 644."
    $Result = 1
}

# Clean up
if (Test-Path $TestFile) { Remove-Item $TestFile -Force }

exit $Result
