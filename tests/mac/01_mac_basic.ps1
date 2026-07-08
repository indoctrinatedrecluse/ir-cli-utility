# Test: mac list local, query vendor, OUI update

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

# --- 1. Local interfaces MAC list ---
Write-Host "Testing local interfaces MAC list..."
$LocalOut = & $Executable mac -l | Out-String
# Since there are local interfaces (physical, virtual, loopback/WSL, etc.), MAC address header must be present.
Assert-Contains $LocalOut "INTERFACE" "Local interfaces header matches"
Assert-Contains $LocalOut "MAC ADDRESS" "Local MAC address header matches"

# --- 2. Query MAC Vendor OUI ---
Write-Host "Testing MAC OUI query..."
$QueryOut = & $Executable mac 00:50:56:12:34:56 | Out-String
Assert-Contains $QueryOut "VMware, Inc." "Resolved MAC query to VMware vendor"

# --- 3. Mutually exclusive switch checks ---
Write-Host "Testing mutually exclusive switches..."
$ErrOut = & $Executable mac -l --update 2>&1 | Out-String
Assert-Contains $ErrOut "Error: Switches --query, --local, and --update are mutually exclusive." "Mutually exclusive checks fail correctly"

# --- 4. Dynamic OUI database update ---
Write-Host "Testing OUI database update..."
$UpdateOut = & $Executable mac --update | Out-String
Assert-Contains $UpdateOut "OUI Database successfully updated" "Update printed database success message"

if ($Passed) {
    Write-Host "`nALL MAC TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME MAC TESTS FAILED"
    exit 1
}
