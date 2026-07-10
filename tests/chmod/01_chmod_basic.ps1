# Test: Basic chmod functionality, including symbolic mode parsing.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$TestFile = New-Item -Path $RepoRoot -Name "temp_chmod_test.txt" -ItemType "file" -Value "chmod test content" -Force
$TestFileAbs = $TestFile.FullName

# --- Test 1: Octal Chmod Read-only ---
Write-Host "Running test: ir chmod 444"
& $Executable chmod 444 $TestFileAbs
$Attributes = [System.IO.File]::GetAttributes($TestFileAbs)
if (($Attributes -band [System.IO.FileAttributes]::ReadOnly) -ne [System.IO.FileAttributes]::ReadOnly) {
    Write-Host "FAIL: Octal chmod 444 did not make file read-only. Attributes: $Attributes"
    Remove-Item $TestFileAbs -Force
    exit 1
}

# --- Test 2: Octal Chmod Writeable ---
Write-Host "Running test: ir chmod 644"
& $Executable chmod 644 $TestFileAbs
$Attributes = [System.IO.File]::GetAttributes($TestFileAbs)
if (($Attributes -band [System.IO.FileAttributes]::ReadOnly) -eq [System.IO.FileAttributes]::ReadOnly) {
    Write-Host "FAIL: Octal chmod 644 did not make file writeable. Attributes: $Attributes"
    Remove-Item $TestFileAbs -Force
    exit 1
}

# --- Test 3: Symbolic Chmod Read-only ---
Write-Host "Running test: ir chmod a-w"
& $Executable chmod a-w $TestFileAbs
$Attributes = [System.IO.File]::GetAttributes($TestFileAbs)
if (($Attributes -band [System.IO.FileAttributes]::ReadOnly) -ne [System.IO.FileAttributes]::ReadOnly) {
    Write-Host "FAIL: Symbolic chmod a-w did not make file read-only. Attributes: $Attributes"
    Remove-Item $TestFileAbs -Force
    exit 1
}

# --- Test 4: Symbolic Chmod Writeable ---
Write-Host "Running test: ir chmod u+w"
& $Executable chmod u+w $TestFileAbs
$Attributes = [System.IO.File]::GetAttributes($TestFileAbs)
if (($Attributes -band [System.IO.FileAttributes]::ReadOnly) -eq [System.IO.FileAttributes]::ReadOnly) {
    Write-Host "FAIL: Symbolic chmod u+w did not make file writeable. Attributes: $Attributes"
    Remove-Item $TestFileAbs -Force
    exit 1
}

Write-Host "PASS: 'chmod' successfully handled octal and symbolic modes."
Remove-Item $TestFileAbs -Force
exit 0
