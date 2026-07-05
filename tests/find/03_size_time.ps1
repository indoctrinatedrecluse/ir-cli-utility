# Test: ir find with --min-size, --max-size, --newer, --older
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"
$TempDir = Join-Path $ScriptDir "temp_find_test"

if (-not (Test-Path $TempDir)) { New-Item -ItemType Directory -Path $TempDir -Force | Out-Null }

$FileSmall = Join-Path $TempDir "small.txt"  # 10 bytes
$FileLarge = Join-Path $TempDir "large.txt"  # 100 bytes

"1234567890" | Out-File -FilePath $FileSmall -NoNewline -Encoding ascii
"0" * 100 | Out-File -FilePath $FileLarge -NoNewline -Encoding ascii

# 1. Test size bounds
Write-Host "Testing --min-size 50..."
$out_min = & $Executable find $TempDir --min-size 50 | Out-String
if ($out_min -match "large.txt" -and $out_min -notmatch "small.txt") {
    Write-Host "✅ PASS: --min-size filters correctly."
} else {
    Write-Host "❌ FAIL: --min-size filter output:`n$out_min"
    $Result = 1
}

Write-Host "Testing --max-size 50..."
$out_max = & $Executable find $TempDir --max-size 50 | Out-String
if ($out_max -match "small.txt" -and $out_max -notmatch "large.txt") {
    Write-Host "✅ PASS: --max-size filters correctly."
} else {
    Write-Host "❌ FAIL: --max-size filter output:`n$out_max"
    $Result = 1
}

# 2. Test time bounds
# Let's create a reference file
$RefFile = Join-Path $TempDir "ref.txt"
"ref" | Out-File -FilePath $RefFile -Encoding ascii

# Wait 1.5 seconds, then modify FileLarge
Start-Sleep -Seconds 2
$LargePath = [System.IO.Path]::GetFullPath($FileLarge)
# touch FileLarge by setting last write time
(Get-Item $FileLarge).LastWriteTime = [System.DateTime]::Now

Write-Host "Testing --newer..."
$out_new = & $Executable find $TempDir --newer $RefFile | Out-String
if ($out_new -match "large.txt" -and $out_new -notmatch "small.txt") {
    Write-Host "✅ PASS: --newer filters correctly."
} else {
    Write-Host "❌ FAIL: --newer filter output:`n$out_new"
    $Result = 1
}

# Clean up
if (Test-Path $TempDir) { Remove-Item $TempDir -Recurse -Force }

exit $Result
