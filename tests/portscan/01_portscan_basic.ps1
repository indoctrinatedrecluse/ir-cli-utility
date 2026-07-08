# Test: portscan targets, ranges, json, timeouts

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

# Spin up a temporary TCP listener on port 9099
Write-Host "Starting temporary TCP listener on port 9099..."
$Listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 9099)
try {
    $Listener.Start()
} catch {
    Write-Host "WARNING: Failed to start TCP listener on port 9099: $_"
}

# --- 1. Basic Port Scan ---
Write-Host "Testing portscan on localhost..."
$ScanOut = & $Executable portscan -p 9090-9105 -t 100 127.0.0.1 | Out-String
Assert-Contains $ScanOut "9099/tcp" "Port 9099 reported as open"

# --- 2. JSON Format Port Scan ---
Write-Host "Testing portscan JSON output..."
$JsonOut = & $Executable portscan -p 9099 --json 127.0.0.1 | Out-String
Assert-Contains $JsonOut '"open_ports": [9099]' "JSON output matches expected open port array"

# Stop the listener
$Listener.Stop()

# --- 3. Invalid parameters ---
Write-Host "Testing portscan parameter validation..."
$ErrOut = & $Executable portscan -p invalid 127.0.0.1 2>&1 | Out-String
Assert-Contains $ErrOut "Error: Invalid port number" "Rejects invalid port specification format"

if ($Passed) {
    Write-Host "`nALL PORTSCAN TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME PORTSCAN TESTS FAILED"
    exit 1
}
