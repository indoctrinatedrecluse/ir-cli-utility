# Test: ir scrape --rate-limit
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"

Write-Host "Testing --rate-limit switch parsing and dry-run..."
# Call with invalid rate-limit
$out_err = & $Executable scrape https://example.com --format pdf --rate-limit abc 2>&1 | Out-String
if ($out_err -match "rate-limit requires a non-negative integer") {
    Write-Host "✅ PASS: Invalid rate-limit rejected."
} else {
    Write-Host "❌ FAIL: Invalid rate-limit output:`n$out_err"
    $Result = 1
}

# Call with valid rate-limit and dry-run
# Since it does a dry-run and example.com might not resolve/respond or might time out,
# we should verify it handles rate limit argument correctly.
$out_ok = & $Executable scrape https://httpbin.org/get --format pdf --rate-limit 500 --dry-run --verbose 2>&1 | Out-String
if ($LASTEXITCODE -eq 0 -or $out_ok -match "would be saved" -or $out_ok -match "Done") {
    Write-Host "✅ PASS: --rate-limit argument parsed and run successfully."
} else {
    Write-Host "❌ FAIL: --rate-limit execution output:`n$out_ok"
    $Result = 1
}

exit $Result
