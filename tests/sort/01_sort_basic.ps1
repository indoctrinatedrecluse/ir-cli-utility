# Test: ir sort basic
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"
$TempFile = "temp_sort_input.txt"

# Create a sample file
@"
banana
apple
Cherry
10
2
100
apple
"@ | Out-File -FilePath $TempFile -Encoding utf8

# 1. Alphanumeric sort
Write-Host "Testing basic sort..."
$out = & $Executable sort $TempFile | Out-String
# Expected (default string sort, case-sensitive):
# 10
# 100
# 2
# Cherry
# apple
# apple
# banana
if ($out -match "10\r?\n100\r?\n2\r?\nCherry\r?\napple\r?\napple\r?\nbanana") {
    Write-Host "✅ PASS: Basic sort matches expected."
} else {
    Write-Host "❌ FAIL: Basic sort output:`n$out"
    $Result = 1
}

# 2. Reverse sort
Write-Host "Testing reverse sort..."
$out_rev = & $Executable sort -r $TempFile | Out-String
if ($out_rev -match "banana\r?\napple\r?\napple\r?\nCherry\r?\n2\r?\n100\r?\n10") {
    Write-Host "✅ PASS: Reverse sort matches expected."
} else {
    Write-Host "❌ FAIL: Reverse sort output:`n$out_rev"
    $Result = 1
}

# 3. Numeric sort
Write-Host "Testing numeric sort..."
$out_num = & $Executable sort -n $TempFile | Out-String
# Expected numeric:
# non-numbers or NaN sort last in Rust's comparison, but let's see.
# na.is_nan() sorts Greater, so numbers first, then NaNs.
# "2", "10", "100" are parsed as numbers.
# "banana", "apple", "Cherry", etc. parse as NaN.
# So "2", "10", "100" should appear first, followed by NaNs.
# Let's verify line order:
# 2
# 10
# 100
# (then Cherry, banana, apple etc)
if ($out_num.StartsWith("2") -and $out_num -match "2\r?\n10\r?\n100") {
    Write-Host "✅ PASS: Numeric sort matches expected."
} else {
    Write-Host "❌ FAIL: Numeric sort output:`n$out_num"
    $Result = 1
}

# 4. Unique sort
Write-Host "Testing unique sort..."
$out_uniq = & $Executable sort -u $TempFile | Out-String
# Count of apple in $out_uniq should be 1
$apple_count = ([regex]::Matches($out_uniq, "apple")).Count
if ($apple_count -eq 1) {
    Write-Host "✅ PASS: Unique sort removed duplicates."
} else {
    Write-Host "❌ FAIL: Unique sort output:`n$out_uniq"
    $Result = 1
}

# 5. Check mode
Write-Host "Testing check mode on sorted file..."
$SortedFile = "temp_sort_sorted.txt"
"2`n10`n100" | Out-File -FilePath $SortedFile -Encoding utf8
& $Executable sort -c -n $SortedFile
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ PASS: Check sorted file passed."
} else {
    Write-Host "❌ FAIL: Check sorted file returned exit code $LASTEXITCODE"
    $Result = 1
}

Write-Host "Testing check mode on unsorted file..."
& $Executable sort -c -n $TempFile 2>&1 | Out-String | Out-Null
# Should fail (non-zero exit code)
if ($LASTEXITCODE -ne 0) {
    Write-Host "✅ PASS: Check unsorted file correctly failed."
} else {
    Write-Host "❌ FAIL: Check unsorted file returned exit code 0"
    $Result = 1
}

# Clean up
if (Test-Path $TempFile) { Remove-Item $TempFile -Force }
if (Test-Path $SortedFile) { Remove-Item $SortedFile -Force }

exit $Result
