# Test: dns action

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

Write-Host "Testing dns resolution of google.com..."
$Output = & $Executable dns google.com | Out-String

Write-Host "Output: $Output"

if ($Output -match "A \(IPv4\)" -and $Output -match "MX \(Mail Server\)") {
    Write-Host "PASS: Successfully queried IPv4 and MX records for google.com."
} else {
    Write-Host "FAIL: Output missing IPv4 or MX records."
    exit 1
}

exit 0
