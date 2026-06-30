# Test: du Action functionality

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
$TestDir = "temp_test_du_01"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Name $TestDir | Out-Null
Set-Location $TestDir

# Create subdirectories
New-Item -ItemType Directory -Name "subdir1" | Out-Null
New-Item -ItemType Directory -Name "subdir2" | Out-Null

# Resolve absolute paths to prevent .NET process CWD mismatch
$file1Path = Join-Path $pwd "subdir1\file1.txt"
$file2Path = Join-Path $pwd "subdir1\file2.txt"
$file3Path = Join-Path $pwd "file3.txt"

# file1.txt -> 13 bytes
$file1Bytes = [System.Text.Encoding]::UTF8.GetBytes("Hello, world!")
[IO.File]::WriteAllBytes($file1Path, $file1Bytes)

# file2.txt -> 1024 bytes (1 KB)
$bytes = New-Object Byte[] 1024
[IO.File]::WriteAllBytes($file2Path, $bytes)

# file3.txt -> 1,048,576 bytes (1 MB)
$bytesMB = New-Object Byte[] 1048576
[IO.File]::WriteAllBytes($file3Path, $bytesMB)

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

# --- Test 1: Default du (KB, directories recursively, child printed first) ---
Write-Host "Running test 1: Default du"
$Output = & $Executable du
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

# Sizes: file1.txt (13B) + file2.txt (1024B) = 1037B -> 2KB in subdir1.
# subdir2 is empty -> 0KB.
# file3.txt (1MB) = 1024KB. Total root = 1024KB + 1037B = 1025KB + 13B -> 1026KB.
$Pass = ($OutputStr -match "2\s+.*subdir1" -and $OutputStr -match "0\s+.*subdir2" -and $OutputStr -match "1026\s+\.")
Check-Test "Default du outputs correct directory sizes in KB" $Pass

# --- Test 2: du with all files (-a) ---
Write-Host "`nRunning test 2: du with all files (-a)"
$Output = & $Executable du -a
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

$Pass = ($OutputStr -match "1\s+.*file1.txt" -and $OutputStr -match "1\s+.*file2.txt" -and $OutputStr -match "1024\s+.*file3.txt")
Check-Test "Tree with -a outputs file sizes" $Pass

# --- Test 3: du with total (-c) ---
Write-Host "`nRunning test 3: du with total (-c)"
$Output = & $Executable du -c
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

$Pass = ($OutputStr -match "1026\s+total")
Check-Test "Tree with -c outputs correct grand total" $Pass

# --- Test 4: du with megabytes (-m) ---
Write-Host "`nRunning test 4: du with megabytes (-m)"
$Output = & $Executable du -m
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

$Pass = ($OutputStr -match "1\s+.*subdir1" -and $OutputStr -match "2\s+\.")
Check-Test "Tree with -m outputs correct sizes in MB" $Pass

# --- Test 5: du with human-readable (-h) ---
Write-Host "`nRunning test 5: du with human-readable (-h)"
$Output = & $Executable du -ah
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

$Pass = ($OutputStr -match "13B\s+.*file1.txt" -and $OutputStr -match "1\.0K\s+.*file2.txt" -and $OutputStr -match "1\.0M\s+.*file3.txt")
Check-Test "Tree with -h formats sizes human-readably" $Pass

# --- Test 6: du with summarize (-s) ---
Write-Host "`nRunning test 6: du with summarize (-s)"
$Output = & $Executable du -s
$OutputStr = $Output -join "`n"
Write-Host "Output:`n$OutputStr"

$Pass = ($OutputStr -match "1026\s+\." -and -not ($OutputStr -match "subdir1"))
Check-Test "Tree with -s outputs only summarizing root" $Pass

# --- Test 7: du incompatible switches (-hk) ---
Write-Host "`nRunning test 7: du incompatible switches (-hk)"
# Redirect error output to capture it
$ErrorOutput = & $Executable du -hk 2>&1
$ErrorStr = $ErrorOutput -join "`n"
Write-Host "Output:`n$ErrorStr"

$Pass = ($ErrorStr -match "exclusive")
Check-Test "Tree with -hk errors out with exclusivity warning" $Pass

# --- Test 8: du incompatible switches (-s -d 1) ---
Write-Host "`nRunning test 8: du incompatible switches (-s -d 1)"
$ErrorOutput = & $Executable du -s -d 1 2>&1
$ErrorStr = $ErrorOutput -join "`n"
Write-Host "Output:`n$ErrorStr"

$Pass = ($ErrorStr -match "Cannot combine")
Check-Test "Tree with -s -d 1 errors out with conflict warning" $Pass

# --- Teardown ---
Set-Location ..
Remove-Item -Recurse -Force $TestDir

exit $Result
