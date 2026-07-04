# Test: Simple ln (hard and soft links)
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$SourceFile = "temp_ln_source.txt"
$HardLinkFile = "temp_ln_hard.txt"
$SymLinkFile = "temp_ln_soft.txt"

# Clean up any leftover from previous runs
if (Test-Path $SourceFile) { Remove-Item $SourceFile }
if (Test-Path $HardLinkFile) { Remove-Item $HardLinkFile }
if (Test-Path $SymLinkFile) { Remove-Item $SymLinkFile }

"original content" | Out-File -FilePath $SourceFile -Encoding ascii -NoNewline

Write-Host "Creating hard link..."
& $Executable ln $SourceFile $HardLinkFile

Write-Host "Creating symbolic link..."
$sym_out = & $Executable ln -s $SourceFile $SymLinkFile 2>&1

$Result = 0

# Check hard link exists
if (Test-Path $HardLinkFile) {
    Write-Host "✅ PASS: Hard link exists."
    # Change content of source and check hard link
    "modified content" | Out-File -FilePath $SourceFile -Encoding ascii -NoNewline
    $hard_content = Get-Content -Path $HardLinkFile -Raw
    if ($hard_content -eq "modified content") {
        Write-Host "✅ PASS: Hard link reflects content change."
    } else {
        Write-Host "❌ FAIL: Hard link did not reflect content change: $hard_content"
        $Result = 1
    }
} else {
    Write-Host "❌ FAIL: Hard link does not exist."
    $Result = 1
}

# Detect privilege error 1314 (missing privilege / Developer Mode)
$HasPrivilege = $true
if ($sym_out -match "1314" -or $sym_out -match "privilege") {
    $HasPrivilege = $false
    Write-Host "⚠️ INFO: Symbolic link creation skipped/failed due to missing Windows Developer Mode or Admin privilege (error 1314)."
}

if ($HasPrivilege) {
    # Check symbolic link exists
    if (Test-Path $SymLinkFile) {
        Write-Host "✅ PASS: Symbolic link exists."
    } else {
        Write-Host "❌ FAIL: Symbolic link does not exist."
        $Result = 1
    }

    # Test overwriting symbolic link with -f (force)
    Write-Host "Testing recreate symlink without -f (should error/fail)..."
    $error_out = & $Executable ln -s $SourceFile $SymLinkFile 2>&1

    Write-Host "Recreating symlink with -f (force)..."
    & $Executable ln -sf $SourceFile $SymLinkFile

    if (Test-Path $SymLinkFile) {
        Write-Host "✅ PASS: Force recreate of symbolic link succeeded."
    } else {
        Write-Host "❌ FAIL: Force recreate failed."
        $Result = 1
    }
}

# Clean up
if (Test-Path $SourceFile) { Remove-Item $SourceFile }
if (Test-Path $HardLinkFile) { Remove-Item $HardLinkFile }
if (Test-Path $SymLinkFile) { Remove-Item $SymLinkFile }

exit $Result
