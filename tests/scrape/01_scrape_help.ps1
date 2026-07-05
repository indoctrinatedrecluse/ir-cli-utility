#!/usr/bin/env pwsh
# Integration tests for 'ir scrape' / 'ir dl' – help routing and basic CLI validation.
# No real network requests are made.

$ErrorActionPreference = "Continue"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "../../")
$Result = 0

# Build first
Write-Host "Building..."
cargo build --manifest-path (Join-Path $ProjectRoot "Cargo.toml") --quiet
if ($LASTEXITCODE -ne 0) { Write-Host "❌ Build failed."; exit 1 }

$Executable = Join-Path $ProjectRoot "target/debug/ir.exe"

# ---------------------------------------------------------------------------
# 1. Help text
# ---------------------------------------------------------------------------
Write-Host "Testing 'ir help scrape'..."
$out = & $Executable help scrape 2>&1 | Out-String
if ($out -match "ir-scrape" -and
    $out -match "\-\-format" -and
    $out -match "\-\-dest" -and
    $out -match "\-\-depth" -and
    $out -match "\-\-max-size" -and
    $out -match "\-\-include-video" -and
    $out -match "\-\-include-audio" -and
    $out -match "\-\-dry-run" -and
    $out -match "documents") {
    Write-Host "✅ PASS: 'ir help scrape' returned correct help text."
} else {
    Write-Host "❌ FAIL: 'ir help scrape' unexpected output: $out"
    $Result = 1
}

# ---------------------------------------------------------------------------
# 2. 'dl' alias routes to scrape help
# ---------------------------------------------------------------------------
Write-Host "Testing 'ir help dl' alias..."
$out2 = & $Executable help dl 2>&1 | Out-String
if ($out2 -match "ir-scrape") {
    Write-Host "✅ PASS: 'ir help dl' correctly routed to scrape help."
} else {
    Write-Host "❌ FAIL: 'ir help dl' unexpected output: $out2"
    $Result = 1
}

# ---------------------------------------------------------------------------
# 3. Missing URL
# ---------------------------------------------------------------------------
Write-Host "Testing 'ir scrape' with no arguments..."
$out3 = & $Executable scrape 2>&1 | Out-String
if ($out3 -match "ir-scrape") {
    Write-Host "✅ PASS: 'ir scrape' (no args) showed help."
} else {
    Write-Host "❌ FAIL: expected help, got: $out3"
    $Result = 1
}

# ---------------------------------------------------------------------------
# 4. Missing --format
# ---------------------------------------------------------------------------
Write-Host "Testing 'ir scrape' missing --format..."
$out4 = & $Executable scrape https://example.com 2>&1 | Out-String
if ($out4 -match "ir-scrape" -or $out4 -match "mandatory" -or $out4 -match "format") {
    Write-Host "✅ PASS: 'ir scrape' without --format showed error/help."
} else {
    Write-Host "❌ FAIL: unexpected output: $out4"
    $Result = 1
}

# ---------------------------------------------------------------------------
# 5. Non-http URL is rejected
# ---------------------------------------------------------------------------
Write-Host "Testing 'ir scrape' with a non-http URL..."
$out5 = & $Executable scrape ftp://example.com --format pdf 2>&1 | Out-String
if ($out5 -match "ir-scrape" -or $out5 -match "http" -or $out5 -match "Error") {
    Write-Host "✅ PASS: non-http URL rejected correctly."
} else {
    Write-Host "❌ FAIL: unexpected output: $out5"
    $Result = 1
}

# ---------------------------------------------------------------------------
# 6. Unknown switch is rejected
# ---------------------------------------------------------------------------
Write-Host "Testing 'ir scrape' with unknown switch..."
$out6 = & $Executable scrape https://example.com --format pdf --badswitch 2>&1 | Out-String
if ($out6 -match "ir-scrape" -or $out6 -match "Unknown") {
    Write-Host "✅ PASS: unknown switch rejected correctly."
} else {
    Write-Host "❌ FAIL: unexpected output: $out6"
    $Result = 1
}

# ---------------------------------------------------------------------------
Write-Host ""
if ($Result -eq 0) {
    Write-Host "All scrape tests passed."
} else {
    Write-Host "Some scrape tests FAILED."
    exit $Result
}
