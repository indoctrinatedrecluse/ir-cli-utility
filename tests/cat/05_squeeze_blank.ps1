# Test: ir cat --squeeze-blank
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"
$TempFile = "temp_squeeze_input.txt"

# Create a sample file with multiple consecutive blank lines
"Line1`n`n`nLine2`n`n`n`nLine3" | Out-File -FilePath $TempFile -Encoding utf8

# Test without squeeze
$out_no = & $Executable cat $TempFile | Out-String
# Test with squeeze
$out_sq = & $Executable cat -s $TempFile | Out-String

# Expected squeezed output should have exactly one blank line between lines:
# Line1`n`nLine2`n`nLine3
# Let's count occurrence of double newlines
if ($out_sq -match "Line1\r?\n\r?\nLine2\r?\n\r?\nLine3" -and $out_sq -notmatch "Line1\r?\n\r?\n\r?\nLine2") {
    Write-Host "✅ PASS: --squeeze-blank collapsed empty lines correctly."
} else {
    Write-Host "❌ FAIL: Squeezed output:`n$out_sq"
    $Result = 1
}

# Clean up
if (Test-Path $TempFile) { Remove-Item $TempFile -Force }

exit $Result
