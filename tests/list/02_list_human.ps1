# Test: ir list with human readable size (-h)
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"
$TempDir = Join-Path $ScriptDir "temp_list_human"

if (Test-Path $TempDir) { Remove-Item -Recurse -Force $TempDir }
New-Item -ItemType Directory -Path $TempDir | Out-Null

# Create a file of exactly 2048 bytes (2.00 KiB)
$File2K = Join-Path $TempDir "file2k.txt"
$bytes = New-Object Byte[] 2048
[System.IO.File]::WriteAllBytes($File2K, $bytes)

# Run list -h
Set-Location $TempDir
$out = & $Executable list -h | Out-String
Set-Location $ScriptDir

if ($out -match "2\.00 KiB") {
    Write-Host "✅ PASS: list -h formatted 2048 bytes as '2.00 KiB'."
} else {
    Write-Host "❌ FAIL: list -h output:`n$out"
    $Result = 1
}

# Clean up
if (Test-Path $TempDir) { Remove-Item -Recurse -Force $TempDir }

exit $Result
