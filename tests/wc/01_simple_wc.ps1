# Test: Simple wc
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# Create a temp file with 3 lines, 6 words, 28 bytes/chars (ASCII)
$TempFile = "temp_wc_test.txt"
"hello world`r`nline two`r`nline three" | Out-File -FilePath $TempFile -Encoding ascii -NoNewline

# Note: "hello world\r\nline two\r\nline three" has:
# Line 1: "hello world" (11 chars + 2 \r\n = 13 bytes)
# Line 2: "line two" (8 chars + 2 \r\n = 10 bytes)
# Line 3: "line three" (10 chars = 10 bytes)
# Total bytes: 33 bytes (depends on PowerShell Out-File encoding behavior, let's read length dynamically)
$BytesCount = (Get-Item $TempFile).Length

Write-Host "Running tests on temp file..."
$output_full = & $Executable wc $TempFile
Write-Host "Output (default): $output_full"

$output_lines = & $Executable wc -l $TempFile
Write-Host "Output (-l): $output_lines"

$output_words = & $Executable wc -w $TempFile
Write-Host "Output (-w): $output_words"

$output_bytes = & $Executable wc -c $TempFile
Write-Host "Output (-c): $output_bytes"

# Stdin test
$output_stdin = "hello world" | & $Executable wc -w
Write-Host "Output (stdin -w): $output_stdin"

# Clean up
if (Test-Path $TempFile) { Remove-Item $TempFile }

$Result = 0
# Basic sanity check of the fields
if ($output_lines -match "\s*2\s+temp_wc_test.txt") {
    Write-Host "✅ PASS: 'ir wc -l' returned 2 newlines."
} else {
    Write-Host "❌ FAIL: 'ir wc -l' returned unexpected output."
    $Result = 1
}

if ($output_words -match "\s*6\s+temp_wc_test.txt") {
    Write-Host "✅ PASS: 'ir wc -w' returned 6 words."
} else {
    Write-Host "❌ FAIL: 'ir wc -w' returned unexpected output."
    $Result = 1
}

if ($output_bytes -match "\s*$BytesCount\s+temp_wc_test.txt") {
    Write-Host "✅ PASS: 'ir wc -c' returned $BytesCount bytes."
} else {
    Write-Host "❌ FAIL: 'ir wc -c' returned unexpected output."
    $Result = 1
}

if ($output_stdin -match "\s*2") {
    Write-Host "✅ PASS: 'stdin | ir wc -w' returned 2 words."
} else {
    Write-Host "❌ FAIL: 'stdin | ir wc -w' returned unexpected output."
    $Result = 1
}

exit $Result
