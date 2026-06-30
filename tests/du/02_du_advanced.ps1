# Test: du Action Advanced Situations (Multiple paths, invalid paths, file inputs, custom depth)

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
$TestDir = "temp_test_du_02"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create subdirectories
New-Item -ItemType Directory -Name "dir1" | Out-Null
New-Item -ItemType Directory -Name "dir1\dir1_sub" | Out-Null
New-Item -ItemType Directory -Name "dir2" | Out-Null

# Resolve absolute paths to prevent .NET process CWD mismatch
$fileAPath = Join-Path $pwd "dir1\dir1_sub\fileA.txt"
$fileBPath = Join-Path $pwd "dir2\fileB.txt"

# fileA.txt -> 512 bytes
$bytesA = New-Object Byte[] 512
[IO.File]::WriteAllBytes($fileAPath, $bytesA)

# fileB.txt -> 2048 bytes (2 KB)
$bytesB = New-Object Byte[] 2048
[IO.File]::WriteAllBytes($fileBPath, $bytesB)

$Result = 0

# Helper function to check test results
function Check-Test {
    param ($Name, $Success)
    if ($Success) {
        Write-Host "✅ PASS: $Name"
    } else {
        Write-Host "❌ FAIL: $Name"
        $script:Result = 1
    }
}

# --- Test 1: Multiple Roots ---
Write-Host "Running test 1: Multiple Roots"
$Output = & $Executable du dir1 dir2
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

# dir1 contains dir1_sub (512B -> 1KB). dir2 contains fileB (2048B -> 2KB).
$Pass = ($OutputStr -match "1\s+.*dir1" -and $OutputStr -match "2\s+.*dir2")
Check-Test "Multiple root paths printed separately with correct size" $Pass

# --- Test 2: Multiple Roots with Total (-c) ---
Write-Host "`nRunning test 2: Multiple Roots with Total (-c)"
$Output = & $Executable du -c dir1 dir2
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

# Total should be dir1 (1KB) + dir2 (2KB) = 3KB
$Pass = ($OutputStr -match "1\s+.*dir1" -and $OutputStr -match "2\s+.*dir2" -and $OutputStr -match "3\s+total")
Check-Test "Multiple roots total matches sum of targets" $Pass

# --- Test 3: Invalid/Non-existent Path ---
Write-Host "`nRunning test 3: Invalid/Non-existent Path"
$ErrorOutput = & $Executable du dir1 non_existent_folder 2>&1
$ErrorStr = $ErrorOutput -join "`n"
Write-Host "Output:`n$ErrorStr"

$Pass = ($ErrorStr -match "non_existent_folder" -and $ErrorStr -match "1\s+.*dir1")
Check-Test "Invalid paths report errors to stderr but valid paths are still printed" $Pass

# --- Test 4: Single File Path Argument ---
Write-Host "`nRunning test 4: Single File Path Argument"
# Run on the file path directly
$Output = & $Executable du dir2\fileB.txt
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

$Pass = ($OutputStr -match "2\s+.*fileB.txt")
Check-Test "Direct file argument outputs its exact KB size" $Pass

# --- Test 5: Depth Limit -d 1 on dir1 ---
Write-Host "`nRunning test 5: Depth Limit -d 1"
$Output = & $Executable du -d 1 dir1
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

# dir1_sub is depth 1 relative to dir1.
$Pass = ($OutputStr -match "1\s+.*dir1_sub" -and $OutputStr -match "1\s+.*dir1")
Check-Test "Depth limit -d 1 lists subdir and parent" $Pass

# --- Test 6: Depth Limit -d 0 on dir1 ---
Write-Host "`nRunning test 6: Depth Limit -d 0"
$Output = & $Executable du -d 0 dir1
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

# Only dir1 should be printed
$Pass = ($OutputStr -match "1\s+.*dir1" -and -not ($OutputStr -match "dir1_sub"))
Check-Test "Depth limit -d 0 acts as summarize" $Pass

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
