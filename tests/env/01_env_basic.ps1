# Test: env action

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: List all env variables ---
Write-Host "Testing ir env..."
$Output = & $Executable env | Out-String
if ($Output -like "*PATH=*") {
    Write-Host "PASS: env listed variables successfully."
} else {
    Write-Host "FAIL: env did not list variables. Output:"
    Write-Host $Output
    exit 1
}

# --- Test 2: Format PATH variable ---
Write-Host "Testing ir env PATH..."
$PathOutput = & $Executable env PATH | Out-String
# Path output should be split by lines and not contain semicolons
if ($PathOutput.Contains(";")) {
    Write-Host "FAIL: PATH variable was not split line-by-line. Output:"
    Write-Host $PathOutput
    exit 1
} else {
    Write-Host "PASS: PATH variable was split line-by-line."
}

# --- Test 3: Search env variables ---
Write-Host "Testing ir env -s CARGO_TEST_VAR..."
$env:CARGO_TEST_VAR = "hello_cargo"
$SearchOutput = & $Executable env -s CARGO_TEST_VAR | Out-String
if ($SearchOutput -like "*CARGO_TEST_VAR=*") {
    Write-Host "PASS: env search filter works."
} else {
    Write-Host "FAIL: env search did not find expected key. Output:"
    Write-Host $SearchOutput
    $env:CARGO_TEST_VAR = $null
    exit 1
}
$env:CARGO_TEST_VAR = $null

# --- Test 4: Query non-existent variable fails ---
Write-Host "Testing non-existent env variable fails..."
& $Executable env NON_EXISTENT_VAR_1234 2>&1 | Out-String
if ($LASTEXITCODE -eq 0) {
    Write-Host "FAIL: Querying non-existent variable did not return failure code."
    exit 1
} else {
    Write-Host "PASS: Querying non-existent variable correctly returned error."
}

# --- Test 5: Case-insensitive lookup ---
Write-Host "Testing case-insensitive env path..."
$PathLowerOutput = & $Executable env path | Out-String
if ($LASTEXITCODE -eq 0 -and $PathLowerOutput.Trim().Length -gt 0 -and !$PathLowerOutput.Contains(";")) {
    Write-Host "PASS: env path lookup is case-insensitive."
} else {
    Write-Host "FAIL: env path lookup failed. Output:"
    Write-Host $PathLowerOutput
    exit 1
}

# --- Test 6: Search with no matches ---
Write-Host "Testing search query with no results..."
$NoMatchOutput = & $Executable env -s ABSOLUTELY_NO_MATCH_EXISTS_1234 | Out-String
if ($LASTEXITCODE -eq 0 -and $NoMatchOutput.Trim().Length -eq 0) {
    Write-Host "PASS: env search with no matches correctly returned empty output."
} else {
    Write-Host "FAIL: env search with no matches output was not empty. Output:"
    Write-Host $NoMatchOutput
    exit 1
}

Write-Host "ALL ENV TESTS PASSED"
exit 0
