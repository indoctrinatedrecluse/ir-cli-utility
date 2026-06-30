# Test: Tree metadata switches, hidden files, and report suppression

# Ensure PowerShell decodes executable output as UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# --- Setup ---
# Dynamically locate the workspace root by climbing up to find Cargo.toml
$RootPath = Get-Location
while ($RootPath -and !(Test-Path (Join-Path $RootPath "Cargo.toml"))) {
    $RootPath = Split-Path $RootPath
}

Write-Host "Building..."
cargo build --quiet
$Executable = Join-Path $RootPath "target/debug/ir.exe"

# Create a temporary directory for the test
$TestDir = "temp_test_tree_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create test files
New-Item -ItemType Directory -Name "subdir" | Out-Null
New-Item -ItemType File -Name "file1.txt" | Out-Null
New-Item -ItemType File -Name ".hidden" | Out-Null # Starts with dot

# Write some content to make size non-zero
Set-Content -Path "file1.txt" -Value "Hello, world!" # 13 bytes plus newline

$Result = 0

# --- Test 1: Hidden Files (default vs -a) ---
Write-Host "Running test 1: Hidden files"
$DefaultOutput = & $Executable tree
$DefaultOutputStr = $DefaultOutput -join "`n"

$AllOutput = & $Executable tree -a
$AllOutputStr = $AllOutput -join "`n"

if ($DefaultOutputStr -match "file1.txt" -and -not ($DefaultOutputStr -match "\.hidden")) {
    Write-Host "✅ Sub-test 1.1 PASS: Default tree hides dotfiles."
} else {
    Write-Host "❌ Sub-test 1.1 FAIL: Default tree did not hide dotfiles."
    $Result = 1
}

if ($AllOutputStr -match "file1.txt" -and $AllOutputStr -match "\.hidden") {
    Write-Host "✅ Sub-test 1.2 PASS: Tree with -a shows dotfiles."
} else {
    Write-Host "❌ Sub-test 1.2 FAIL: Tree with -a did not show dotfiles."
    $Result = 1
}

# --- Test 2: Permissions (-p) ---
Write-Host "`nRunning test 2: Permissions (-p)"
$Output = & $Executable tree -p
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

# Verify permissions formatting like `[d...]` (Linux) or `[d-r-h-s]`/`[d-rw-??????]` (Windows fallback)
if ($OutputStr -match "\[[dl-][r-][w-][x-][r-][w-][x-][r-][w-][x-]\]" -or $OutputStr -match "\[[d-][rw-]{3}\?{6}\]") {
    Write-Host "✅ Test 2 PASS: Tree with -p contains permission brackets."
} else {
    Write-Host "❌ Test 2 FAIL: Permissions format did not match expected pattern."
    $Result = 1
}

# --- Test 3: Sizes (-s and -h) ---
Write-Host "`nRunning test 3: Sizes (-s and -h)"
$OutputS = & $Executable tree -s
$OutputSStr = $OutputS -join "`n"

$OutputH = & $Executable tree -h
$OutputHStr = $OutputH -join "`n"

if ($OutputSStr -match "\[\d+\]") {
    Write-Host "✅ Sub-test 3.1 PASS: Tree with -s contains raw size brackets."
} else {
    Write-Host "❌ Sub-test 3.1 FAIL: Tree with -s does not contain size formatting."
    $Result = 1
}

if ($OutputHStr -match "\[\d+(\.\d+)?\s*(B|KB|MB|GB)\]") {
    Write-Host "✅ Sub-test 3.2 PASS: Tree with -h contains human-readable size brackets."
} else {
    Write-Host "❌ Sub-test 3.2 FAIL: Tree with -h does not contain human-readable size formatting."
    $Result = 1
}

# --- Test 4: Full Path (-f) ---
Write-Host "`nRunning test 4: Full path (-f)"
$Output = & $Executable tree -f
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

if ($OutputStr -match "(\.\\|/|temp_test_tree_02)file1\.txt") {
    Write-Host "✅ Test 4 PASS: Tree with -f outputs path prefixes."
} else {
    Write-Host "❌ Test 4 FAIL: Tree with -f did not output path prefix."
    $Result = 1
}

# --- Test 5: No Report (--noreport) ---
Write-Host "`nRunning test 5: No report (--noreport)"
$Output = & $Executable tree --noreport
$OutputStr = $Output -join "`n"

if (-not ($OutputStr -match "directories,\s+\d+\s+files")) {
    Write-Host "✅ Test 5 PASS: Tree with --noreport omitted the final count summary."
} else {
    Write-Host "❌ Test 5 FAIL: Tree with --noreport still contained the count summary."
    $Result = 1
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
