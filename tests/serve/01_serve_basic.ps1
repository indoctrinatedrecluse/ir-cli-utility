# Test: serve static file server, MIME types, directory listing, traversal protection

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

# Create a temporary directory with files to serve
$TempDir = Join-Path $env:TEMP "ir_serve_test_dir_$(Get-Random)"
$null = New-Item -ItemType Directory -Path $TempDir -Force

$IndexContent = "<h1>Welcome to Local Serve Test Page</h1>"
Set-Content -Path (Join-Path $TempDir "index.html") -Value $IndexContent

$TxtContent = "Hello from text file."
Set-Content -Path (Join-Path $TempDir "test.txt") -Value $TxtContent

# Start ir serve in a background job on port 18080
Write-Host "Starting local HTTP server on port 18080 in background..."
$Job = Start-Job -ScriptBlock {
    param($Exe, $Dir)
    & $Exe serve -p 18080 $Dir
} -ArgumentList $Executable, $TempDir

# Give it 2 seconds to initialize and bind
Start-Sleep -Seconds 2

try {
    # 1. Fetch index.html
    Write-Host "Testing index.html lookup..."
    $Response = Invoke-WebRequest -Uri "http://127.0.0.1:18080/" -UseBasicParsing
    Assert-Contains $Response.Content "Welcome to Local Serve Test Page" "Default index.html served correctly"

    # 2. Fetch test.txt and check headers
    Write-Host "Testing test.txt lookup..."
    $ResponseTxt = Invoke-WebRequest -Uri "http://127.0.0.1:18080/test.txt" -UseBasicParsing
    Assert-Contains $ResponseTxt.Content "Hello from text file." "Static text file served correctly"
    Assert-Contains $ResponseTxt.Headers["Content-Type"] "text/plain" "MIME type header matches text/plain"

    # 3. Directory listing: Delete index.html and request directory index
    Write-Host "Testing auto-generated directory listing..."
    Remove-Item -Path (Join-Path $TempDir "index.html") -Force
    $ResponseDir = Invoke-WebRequest -Uri "http://127.0.0.1:18080/" -UseBasicParsing
    Assert-Contains $ResponseDir.Content "Index of /" "Directory listing header matches"
    Assert-Contains $ResponseDir.Content "test.txt" "Directory listing contains test.txt entry"

    # 4. Traversal protection: Attempt to escape directory using dot-dot path
    Write-Host "Testing traversal protection block..."
    # Invoke-WebRequest automatically normalizes /../. We can use a raw TcpClient or WebClient to send un-normalized path.
    # Alternatively, send %2e%2e%2f to check server-side decoding and blocking
    try {
        $null = Invoke-WebRequest -Uri "http://127.0.0.1:18080/%2e%2e%2fCargo.toml" -UseBasicParsing
        Write-Host "FAIL: Traversal block (Expected request to fail with 403)"
        $Passed = $false
    } catch {
        $StatusCode = $_.Exception.Response.StatusCode.value__
        if ($StatusCode -eq 403 -or $_.Exception.Message -like "*403*") {
            Write-Host "PASS: Traversal block correctly returned 403 Forbidden"
        } else {
            Write-Host "FAIL: Traversal block returned unexpected error: $_"
            $Passed = $false
        }
    }
} finally {
    # Stop background job and clean up files
    Write-Host "Stopping background server..."
    Stop-Job $Job
    Remove-Job $Job
    Remove-Item -Path $TempDir -Recurse -Force
}

if ($Passed) {
    Write-Host "`nALL SERVE TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME SERVE TESTS FAILED"
    exit 1
}
