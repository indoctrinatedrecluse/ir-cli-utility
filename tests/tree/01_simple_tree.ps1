# Test: Simple tree structure listing

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

# Define Unicode drawing characters using character codes to prevent file encoding issues in PowerShell
$TreeMid = [char]0x251c + [char]0x2500 + [char]0x2500 # ├──
$TreeEnd = [char]0x2514 + [char]0x2500 + [char]0x2500 # └──

# Create a temporary directory for the test
$TestDir = "temp_test_tree_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create a temporary folder structure
New-Item -ItemType Directory -Name "subdir1" | Out-Null
New-Item -ItemType Directory -Name "subdir2" | Out-Null
New-Item -ItemType File -Name "file2.log" | Out-Null
New-Item -ItemType File -Path "subdir1\file1.txt" | Out-Null

$Result = 0

# --- Test 1: Default Tree ---
Write-Host "Running test 1: Default tree"
$Output = & $Executable tree
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

if ($OutputStr -match "subdir1" -and $OutputStr -match "subdir2" -and $OutputStr -match "file1.txt" -and $OutputStr -match "file2.log" -and $OutputStr.Contains($TreeMid) -and $OutputStr.Contains($TreeEnd)) {
    Write-Host "✅ Test 1 PASS: Default tree has correct structure."
} else {
    Write-Host "❌ Test 1 FAIL: Default tree did not match expected output."
    $Result = 1
}

# --- Test 2: Directories Only (-d) ---
Write-Host "`nRunning test 2: Directories only (-d)"
$Output = & $Executable tree -d
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

if ($OutputStr -match "subdir1" -and $OutputStr -match "subdir2" -and -not ($OutputStr -match "file1.txt") -and -not ($OutputStr -match "file2.log")) {
    Write-Host "✅ Test 2 PASS: Dirs-only tree correctly filtered out files."
} else {
    Write-Host "❌ Test 2 FAIL: Dirs-only tree contains files or incorrect listing."
    $Result = 1
}

# --- Test 3: Depth Limit (-L 1) ---
Write-Host "`nRunning test 3: Depth limit (-L 1)"
$Output = & $Executable tree -L 1
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

if ($OutputStr -match "subdir1" -and $OutputStr -match "subdir2" -and -not ($OutputStr -match "file1.txt") -and $OutputStr -match "file2.log") {
    Write-Host "✅ Test 3 PASS: Depth limit tree correctly omitted deeper files."
} else {
    Write-Host "❌ Test 3 FAIL: Depth limit tree did not respect depth limit."
    $Result = 1
}

# --- Test 4: Omit Indentation (-i) ---
Write-Host "`nRunning test 4: Omit indentation (-i)"
$Output = & $Executable tree -i
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

if (-not $OutputStr.Contains($TreeMid) -and -not $OutputStr.Contains($TreeEnd) -and $OutputStr -match "subdir1" -and $OutputStr -match "file1.txt") {
    Write-Host "✅ Test 4 PASS: No-indent tree omitted drawing characters."
} else {
    Write-Host "❌ Test 4 FAIL: No-indent tree still contains drawing characters or failed listing."
    $Result = 1
}

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
