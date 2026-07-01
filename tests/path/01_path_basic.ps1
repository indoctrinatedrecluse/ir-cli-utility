# Test: path action

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# 1. Test listing
Write-Host "Testing path list..."
$ListOut = & $Executable path | Out-String
if ($ListOut -match "User Registry PATH" -and $ListOut -match "Active Process PATH") {
    Write-Host "PASS: Path list contains User Registry and Active Process PATH blocks."
} else {
    Write-Host "FAIL: Missing header blocks in path list."
    exit 1
}

# 2. Test adding path
Write-Host "Testing path add..."
$AddDir = "C:\temp_ir_test_dir_path"
$AddOut = & $Executable path -a $AddDir | Out-String
if ($AddOut -match "Success: Added") {
    Write-Host "PASS: Successfully added directory to user PATH."
} else {
    Write-Host "FAIL: Failed to add directory to user PATH. Output: $AddOut"
    exit 1
}

# 3. Verify it was added
$ListOut2 = & $Executable path | Out-String
if ($ListOut2 -match "temp_ir_test_dir_path") {
    Write-Host "PASS: Verified directory exists in registry PATH."
} else {
    Write-Host "FAIL: Directory not found in PATH list."
    & $Executable path -r $AddDir | Out-String
    exit 1
}

# 4. Test removing path
Write-Host "Testing path remove..."
$RemoveOut = & $Executable path -r $AddDir | Out-String
if ($RemoveOut -match "Success: Removed") {
    Write-Host "PASS: Successfully removed directory from user PATH."
} else {
    Write-Host "FAIL: Failed to remove directory from user PATH."
    exit 1
}

# 5. Verify it was removed
$ListOut3 = & $Executable path | Out-String
if ($ListOut3 -notmatch "temp_ir_test_dir_path") {
    Write-Host "PASS: Verified directory was removed from registry PATH."
} else {
    Write-Host "FAIL: Directory still exists in registry PATH after removal."
    exit 1
}

exit 0
