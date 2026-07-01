# Test: echo action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"
$TestDir = "temp_test_echo"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$OutFile = Join-Path $TestDir "output.txt"

# --- Test 1: Simple print ---
Write-Host "Testing ir echo basic..."
$Output = & $Executable echo hello world | Out-String
if ($Output.Trim() -eq "hello world") {
    Write-Host "PASS: Simple print output matches."
} else {
    Write-Host "FAIL: Simple print output mismatch: '$($Output.Trim())'"
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 2: Escape interpretation ---
Write-Host "Testing ir echo -e..."
$EscapeOut = & $Executable echo -e 'line1\nline2\x41' | Out-String
if ($EscapeOut -like "*line1`r`nline2A*") {
    Write-Host "PASS: Escape sequences interpreted successfully."
} else {
    Write-Host "FAIL: Escape sequences mismatch. Output:"
    Write-Host $EscapeOut
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 3: Redirection write > ---
Write-Host "Testing ir echo > redirection..."
& $Executable echo "first line" '>' $OutFile | Out-String
if (Test-Path $OutFile) {
    $Content = Get-Content $OutFile | Out-String
    if ($Content.Trim() -eq "first line") {
        Write-Host "PASS: Redirection > wrote successfully."
    } else {
        Write-Host "FAIL: Content mismatch: '$($Content.Trim())'"
        Remove-Item -Recurse -Force $TestDir
        exit 1
    }
} else {
    Write-Host "FAIL: Redirection file not created."
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 4: Redirection append >> ---
Write-Host "Testing ir echo >> redirection..."
& $Executable echo "second line" '>>' $OutFile | Out-String
$Content = Get-Content $OutFile | ForEach-Object { $_.Trim() }
if ($Content.Count -eq 2 -and $Content[1] -eq "second line") {
    Write-Host "PASS: Redirection >> appended successfully."
} else {
    Write-Host "FAIL: Append content mismatch. Lines count: $($Content.Count)"
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Test 5: Redirection missing file argument fails ---
Write-Host "Testing missing file argument for redirection fails..."
& $Executable echo "text" '>' 2>$null | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "PASS: Redirection without file failed correctly."
} else {
    Write-Host "FAIL: Redirection without file did not return error code."
    Remove-Item -Recurse -Force $TestDir
    exit 1
}

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir
Write-Host "ALL ECHO TESTS PASSED"
exit 0
