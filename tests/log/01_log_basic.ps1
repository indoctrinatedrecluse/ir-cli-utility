# Test: log parsing, filtering and metrics summary

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"

$Passed = $true
function Assert-Equal($Actual, $Expected, $Msg) {
    if ($Actual.Trim() -eq $Expected.Trim()) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected '$Expected', got '$Actual')"
        $global:Passed = $false
    }
}
function Assert-Contains($Actual, $Expected, $Msg) {
    if ($Actual.Contains($Expected)) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected to contain '$Expected')"
        $global:Passed = $false
    }
}

# Write mock log files
$CombinedLog = Join-Path $ScriptDir "mock_combined.log"
@'
127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 200 2326 "http://referer.com" "Mozilla/5.0"
10.0.0.2 - - [10/Oct/2000:13:56:01 -0700] "POST /api/login HTTP/1.0" 401 150 "-" "Mozilla/5.0"
10.0.0.2 - - [10/Oct/2000:13:56:45 -0700] "GET /api/data HTTP/1.0" 200 8500 "http://referer.com" "Mozilla/5.0"
192.168.1.5 - - [10/Oct/2000:13:57:12 -0700] "GET /non-existent HTTP/1.0" 404 0 "-" "Mozilla/5.0"
'@ | Set-Content $CombinedLog -NoNewline

$JsonLog = Join-Path $ScriptDir "mock_json.log"
@'
{"ip":"127.0.0.1","time":"10/Oct/2000","method":"GET","path":"/index.html","status":200,"size":2326}
{"ip":"10.0.0.2","time":"10/Oct/2000","method":"POST","path":"/api/login","status":401,"size":150}
{"ip":"10.0.0.2","time":"10/Oct/2000","method":"GET","path":"/api/data","status":200,"size":8500}
{"ip":"192.168.1.5","time":"10/Oct/2000","method":"GET","path":"/non-existent","status":404,"size":0}
'@ | Set-Content $JsonLog -NoNewline

# --- 1. Basic Log Parsing ---
Write-Host "Testing basic log parsing..."
$ClfOut = & $Executable log $CombinedLog | Out-String
Assert-Contains $ClfOut '127.0.0.1 - [10/Oct/2000:13:55:36 -0700] "GET /index.html" 200 2326' "Parse Combined/CLF log"

$JsonOut = & $Executable log $JsonLog --format json | Out-String
Assert-Contains $JsonOut '127.0.0.1 - [10/Oct/2000] "GET /index.html" 200 2326' "Parse JSON format log"

# --- 2. Query Filtering ---
Write-Host "Testing query filtering..."
$QueryStatus = & $Executable log $CombinedLog -q "status >= 400" | Out-String
Assert-Contains $QueryStatus '10.0.0.2 - [10/Oct/2000:13:56:01 -0700] "POST /api/login" 401 150' "Filter status >= 400 (POST)"
Assert-Contains $QueryStatus '192.168.1.5 - [10/Oct/2000:13:57:12 -0700] "GET /non-existent" 404 0' "Filter status >= 400 (404)"
if ($QueryStatus.Contains(" 200 ")) {
    Write-Host "FAIL: Filter status >= 400 should not contain status 200"
    $Passed = $false
} else {
    Write-Host "PASS: Filter status >= 400 did not include status 200"
}

$QueryPath = & $Executable log $CombinedLog -q "path contains /api" | Out-String
Assert-Contains $QueryPath "/api/login" "Filter path contains /api (login)"
Assert-Contains $QueryPath "/api/data" "Filter path contains /api (data)"
if ($QueryPath.Contains("index.html")) {
    Write-Host "FAIL: Filter path contains /api should not include /index.html"
    $Passed = $false
} else {
    Write-Host "PASS: Filter path contains /api did not include /index.html"
}

# --- 3. Log Statistics Metrics ---
Write-Host "Testing log stats reports..."
$StatsOut = & $Executable log $CombinedLog --stats | Out-String
Assert-Contains $StatsOut "Total Requests:      4" "Stats total requests count"
Assert-Contains $StatsOut "Failed Requests:     2 (50.00%)" "Stats failed requests count"
Assert-Contains $StatsOut "Data Transferred:    10976 bytes" "Stats total data bytes count"
Assert-Contains $StatsOut "10.0.0.2: 2 requests" "Stats top client IPs list"
Assert-Contains $StatsOut "HTTP 200: 2 requests" "Stats status distribution"

# Clean up mock logs
Remove-Item $CombinedLog
Remove-Item $JsonLog

# --- 4. Error Handling ---
Write-Host "Testing error handling..."

$ErrQuery = "" | & $Executable log -q "status = 200" 2>&1 | Out-String
if ($ErrQuery.Contains("Error: Invalid query expression syntax") -and $LASTEXITCODE -ne 0) {
    Write-Host "PASS: Invalid query filter syntax error detected"
} else {
    Write-Host "FAIL: Invalid query filter syntax should return error"
    $Passed = $false
}

$ErrFormat = "" | & $Executable log -f invalid_fmt 2>&1 | Out-String
if ($ErrFormat.Contains("Error: Invalid log format") -and $LASTEXITCODE -ne 0) {
    Write-Host "PASS: Invalid log format error detected"
} else {
    Write-Host "FAIL: Invalid log format should return error"
    $Passed = $false
}

if ($Passed) {
    Write-Host "`nALL LOG INTEGRATION TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME LOG INTEGRATION TESTS FAILED"
    exit 1
}
