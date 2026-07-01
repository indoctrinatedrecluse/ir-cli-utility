# Test: clip action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: Write to clip via redirection ---
Write-Host "Testing echo redirection to clip..."
& $Executable echo "clip content 1" '>' clip | Out-String
$ClipText = Get-Clipboard
if ($ClipText.Trim() -eq "clip content 1") {
    Write-Host "PASS: Redirection > clip wrote successfully."
} else {
    Write-Host "FAIL: Clipboard content mismatch: '$($ClipText.Trim())'"
    exit 1
}

# --- Test 2: Append to clip via redirection ---
Write-Host "Testing echo append redirection to clip..."
& $Executable echo "clip content 2" '>>' clip | Out-String
$ClipTextLines = Get-Clipboard | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" }
if ($ClipTextLines.Count -eq 2 -and $ClipTextLines[1] -eq "clip content 2") {
    Write-Host "PASS: Redirection >> clip appended successfully."
} else {
    Write-Host "FAIL: Clipboard append mismatch. Lines count: $($ClipTextLines.Count). Elements: '$($ClipTextLines -join ',')'"
    exit 1
}

# --- Test 3: Cat redirection to clip ---
Write-Host "Testing cat redirection to clip..."
$TempFile = "temp_cat_clip.txt"
"hello cat clip" | Out-File -FilePath $TempFile -Encoding utf8
& $Executable cat $TempFile '>' clip | Out-String
$ClipText = Get-Clipboard
if ($ClipText.Trim() -eq "hello cat clip") {
    Write-Host "PASS: Cat redirection > clip wrote successfully."
} else {
    Write-Host "FAIL: Cat clipboard content mismatch: '$($ClipText.Trim())'"
    Remove-Item -Force $TempFile
    exit 1
}
Remove-Item -Force $TempFile

# --- Test 4: Clear clip ---
Write-Host "Testing ir clip --clear..."
& $Executable clip --clear | Out-String
$ClipText = Get-Clipboard
if ([string]::IsNullOrEmpty($ClipText)) {
    Write-Host "PASS: Clipboard cleared successfully."
} else {
    Write-Host "FAIL: Clipboard was not cleared."
    exit 1
}

Write-Host "ALL CLIP TESTS PASSED"
exit 0
