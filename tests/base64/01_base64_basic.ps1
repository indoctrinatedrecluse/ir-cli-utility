# Test: base64 action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: Simple Base64 encode ---
Write-Host "Testing ir base64 encode..."
$Output = "hello" | & $Executable base64 | Out-String
# PowerShell adds a newline to piped strings
if ($Output.Trim() -eq "aGVsbG8NCg==" -or $Output.Trim() -eq "aGVsbG8K") {
    Write-Host "PASS: Simple encode output matches."
} else {
    Write-Host "FAIL: Simple encode output mismatch: '$($Output.Trim())'"
    exit 1
}

# --- Test 2: Simple Base64 decode ---
Write-Host "Testing ir base64 decode..."
$Decoded = "aGVsbG8=" | & $Executable base64 -d | Out-String
if ($Decoded.Trim() -eq "hello") {
    Write-Host "PASS: Simple decode output matches."
} else {
    Write-Host "FAIL: Simple decode output mismatch: '$($Decoded.Trim())'"
    exit 1
}

# --- Test 3: Base64 encode URL-safe unpadded ---
Write-Host "Testing ir base64 -u -n (URL-safe unpadded)..."
# "+" in standard becomes "-" in URL-safe, "/" becomes "_"
# We pass binary that results in standard ending with "+" or "/"
# E.g. [byte[]](251, 255) -> standard: "+/8=", URL-safe: "-_8"
$TestFile = "temp_b64_test.bin"
[System.IO.File]::WriteAllBytes($TestFile, @(251, 255))

$UrlSafeOut = & $Executable base64 -u -n $TestFile | Out-String
if ($UrlSafeOut.Trim() -eq "-_8") {
    Write-Host "PASS: URL-safe unpadded encode matches."
} else {
    Write-Host "FAIL: URL-safe unpadded mismatch: '$($UrlSafeOut.Trim())'"
    Remove-Item -Force $TestFile
    exit 1
}
Remove-Item -Force $TestFile

Write-Host "ALL BASE64 TESTS PASSED"
exit 0
