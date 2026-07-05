# Test: ir grep -A, -B, -C context lines
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"
$TempFile = "temp_grep_context.txt"

# Sample file
@"
one
two
match
three
four
match
five
six
"@ | Out-File -FilePath $TempFile -Encoding utf8

# Test 1: Before context -B 1
Write-Host "Testing -B 1..."
$out_b = & $Executable grep -B 1 "match" $TempFile | Out-String
if ($out_b -match "temp_grep_context\.txt-two\r?\ntemp_grep_context\.txt:match\r?\n--\r?\ntemp_grep_context\.txt-four\r?\ntemp_grep_context\.txt:match") {
    Write-Host "✅ PASS: -B 1 matches expected."
} else {
    Write-Host "❌ FAIL: -B 1 output:`n$out_b"
    $Result = 1
}

# Test 2: After context -A 2
Write-Host "Testing -A 2..."
$out_a = & $Executable grep -A 2 "match" $TempFile | Out-String
if ($out_a -match "temp_grep_context\.txt:match\r?\ntemp_grep_context\.txt-three\r?\ntemp_grep_context\.txt-four\r?\ntemp_grep_context\.txt:match\r?\ntemp_grep_context\.txt-five\r?\ntemp_grep_context\.txt-six") {
    Write-Host "✅ PASS: -A 2 matches expected (overlap merged)."
} else {
    Write-Host "❌ FAIL: -A 2 output:`n$out_a"
    $Result = 1
}

# Test 3: Context -C 1
Write-Host "Testing -C 1..."
$out_c = & $Executable grep -C 1 "match" $TempFile | Out-String
if ($out_c -match "temp_grep_context\.txt-two\r?\ntemp_grep_context\.txt:match\r?\ntemp_grep_context\.txt-three\r?\ntemp_grep_context\.txt-four\r?\ntemp_grep_context\.txt:match\r?\ntemp_grep_context\.txt-five") {
    Write-Host "✅ PASS: -C 1 matches expected."
} else {
    Write-Host "❌ FAIL: -C 1 output:`n$out_c"
    $Result = 1
}

# Clean up
if (Test-Path $TempFile) { Remove-Item $TempFile -Force }

exit $Result
