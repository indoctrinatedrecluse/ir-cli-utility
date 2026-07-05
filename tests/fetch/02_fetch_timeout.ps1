# Test: fetch timeout and redirection options
$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"

# 1. Invalid timeout
Write-Host "Testing invalid timeout..."
$out_err = & $Executable fetch --timeout abc https://api.ipify.org 2>&1 | Out-String
if ($out_err -match "timeout requires a non-negative integer") {
    Write-Host "✅ PASS: Invalid timeout rejected correctly."
} else {
    Write-Host "❌ FAIL: Invalid timeout output:`n$out_err"
    $Result = 1
}

# 2. Timeout triggering (HTTP delay)
Write-Host "Testing timeout trigger on slow response..."
$start_time = Get-Date
& $Executable fetch --timeout 2 https://httpbin.org/delay/5 2>&1 | Out-String | Out-Null
$duration = ((Get-Date) - $start_time).TotalSeconds

if ($LASTEXITCODE -ne 0 -and $duration -lt 4) {
    Write-Host "✅ PASS: Request timed out correctly after $($duration) seconds."
} else {
    Write-Host "❌ FAIL: Request did not time out as expected (exit code: $LASTEXITCODE, duration: $duration s)."
    $Result = 1
}

# 3. No-follow-redirects
Write-Host "Testing --no-follow-redirects..."
# https://httpbin.org/redirect-to?url=https://api.ipify.org redirects.
# With --no-follow-redirects, it should return a 302 status code and print it if -i is used.
$out_redir = & $Executable fetch -i --no-follow-redirects "https://httpbin.org/redirect-to?url=https%3A%2F%2Fapi.ipify.org&status_code=302" | Out-String
if ($out_redir -match "HTTP/1.1 302" -or $out_redir -match "302 Found") {
    Write-Host "✅ PASS: Redirect was not followed."
} else {
    Write-Host "❌ FAIL: Redirect test output:`n$out_redir"
    $Result = 1
}

# 4. Progress bar parsing
Write-Host "Testing progress bar flag..."
$TempOut = "temp_fetch_progress.txt"
if (Test-Path $TempOut) { Remove-Item $TempOut -Force }

& $Executable fetch -p -o $TempOut https://api.ipify.org 2>&1 | Out-String | Out-Null
if ($LASTEXITCODE -eq 0 -and (Test-Path $TempOut)) {
    Write-Host "✅ PASS: Progress bar parsing/execution succeeded."
} else {
    Write-Host "❌ FAIL: Fetch with progress bar failed."
    $Result = 1
}
if (Test-Path $TempOut) { Remove-Item $TempOut -Force }

exit $Result
