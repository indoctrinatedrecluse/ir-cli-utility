#!/usr/bin/env bash
# Integration tests for 'ir scrape' / 'ir dl' – no real network requests.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULT=0

echo "Building..."
cargo build --manifest-path "$PROJECT_ROOT/Cargo.toml" --quiet
EXECUTABLE="$PROJECT_ROOT/target/debug/ir"

# 1. Help text
echo "Testing 'ir help scrape'..."
out=$("$EXECUTABLE" help scrape 2>&1 || true)
if echo "$out" | grep -q "ir-scrape" && \
   echo "$out" | grep -q -- "--format"  && \
   echo "$out" | grep -q -- "--dest"    && \
   echo "$out" | grep -q -- "--depth"   && \
   echo "$out" | grep -q -- "--max-size" && \
   echo "$out" | grep -q -- "--include-video" && \
   echo "$out" | grep -q -- "--dry-run" && \
   echo "$out" | grep -q "documents"; then
    echo "✅ PASS: 'ir help scrape' correct."
else
    echo "❌ FAIL: 'ir help scrape' unexpected: $out"
    RESULT=1
fi

# 2. Alias
echo "Testing 'ir help dl' alias..."
out2=$("$EXECUTABLE" help dl 2>&1 || true)
if echo "$out2" | grep -q "ir-scrape"; then
    echo "✅ PASS: 'ir help dl' alias routes correctly."
else
    echo "❌ FAIL: unexpected: $out2"
    RESULT=1
fi

# 3. Missing URL
echo "Testing 'ir scrape' with no arguments..."
out3=$("$EXECUTABLE" scrape 2>&1 || true)
if echo "$out3" | grep -q "ir-scrape"; then
    echo "✅ PASS: missing URL shows help."
else
    echo "❌ FAIL: unexpected: $out3"
    RESULT=1
fi

# 4. Missing --format
echo "Testing 'ir scrape' without --format..."
out4=$("$EXECUTABLE" scrape https://example.com 2>&1 || true)
if echo "$out4" | grep -q "ir-scrape" || echo "$out4" | grep -q "format"; then
    echo "✅ PASS: missing --format shows error/help."
else
    echo "❌ FAIL: unexpected: $out4"
    RESULT=1
fi

# 5. Non-http URL
echo "Testing 'ir scrape' with ftp:// URL..."
out5=$("$EXECUTABLE" scrape ftp://example.com --format pdf 2>&1 || true)
if echo "$out5" | grep -qi "http\|error\|ir-scrape"; then
    echo "✅ PASS: ftp URL rejected."
else
    echo "❌ FAIL: unexpected: $out5"
    RESULT=1
fi

# 6. Unknown switch
echo "Testing 'ir scrape' unknown switch..."
out6=$("$EXECUTABLE" scrape https://example.com --format pdf --badswitch 2>&1 || true)
if echo "$out6" | grep -qi "unknown\|ir-scrape"; then
    echo "✅ PASS: unknown switch rejected."
else
    echo "❌ FAIL: unexpected: $out6"
    RESULT=1
fi

echo ""
if [ $RESULT -eq 0 ]; then echo "All scrape tests passed."; else echo "Some scrape tests FAILED."; exit $RESULT; fi
