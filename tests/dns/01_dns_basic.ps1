# Test: dns query options, types, short, reverse, trace

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"

$Passed = $true
function Assert-Contains($Actual, $Expected, $Msg) {
    if ($Actual.Contains($Expected)) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected to contain '$Expected')"
        $global:Passed = $false
    }
}

# --- 1. Basic DNS Query ---
Write-Host "Testing basic DNS query..."
$DnsOut = & $Executable dns -s 8.8.8.8 -t A google.com | Out-String
Assert-Contains $DnsOut "A:" "Query A record header matches"
if ($DnsOut -match '\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}') {
    Write-Host "PASS: Query A record resolved IP address"
} else {
    Write-Host "FAIL: Query A record did not resolve IP address"
    $Passed = $false
}

# --- 2. Short DNS Query ---
Write-Host "Testing short DNS query..."
$ShortOut = & $Executable dns -s 8.8.8.8 -t A --short google.com | Out-String
if ($ShortOut.Trim() -match '^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$') {
    Write-Host "PASS: Short query returns clean IP format"
} else {
    Write-Host "FAIL: Short query did not return a clean IP address ($ShortOut)"
    $Passed = $false
}

# --- 3. Reverse DNS Lookup ---
Write-Host "Testing reverse DNS PTR lookup..."
$ReverseOut = & $Executable dns -s 8.8.8.8 -x 8.8.8.8 | Out-String
Assert-Contains $ReverseOut "dns.google" "Reverse lookup resolved IP to hostname"

# --- 4. DNS Tracing ---
Write-Host "Testing DNS tracing..."
$TraceOut = & $Executable dns --trace wikipedia.org | Out-String
Assert-Contains $TraceOut "Tracing delegation path" "Trace output header matches"
Assert-Contains $TraceOut "Authoritative Answer Received" "Trace reached authoritative answer"

if ($Passed) {
    Write-Host "`nALL DNS TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME DNS TESTS FAILED"
    exit 1
}
