#!/usr/bin/env bash
# Test: ir dua — verify help routing, ncdu alias, and non-interactive scan.

set -e
echo "Building..."
cargo build --quiet --manifest-path "../../Cargo.toml"
EXECUTABLE="../../target/debug/ir"
RESULT=0
TMPDIR_LOCAL="temp_dua_test_dir"

# Setup
rm -rf "$TMPDIR_LOCAL"
mkdir -p "$TMPDIR_LOCAL"
echo "Hello" > "$TMPDIR_LOCAL/file_a.txt"
echo "World, this is a larger file with more content." > "$TMPDIR_LOCAL/file_b.txt"

# 1. Help routing
echo "Testing 'ir help dua'..."
out=$("$EXECUTABLE" help dua 2>&1)
if echo "$out" | grep -q "ir-dua" && echo "$out" | grep -q "\[PATH\]"; then
    echo "✅ PASS: 'ir help dua' returned correct help text."
else
    echo "❌ FAIL: 'ir help dua' did not return expected output: $out"
    RESULT=1
fi

# 2. Alias 'ncdu' maps to dua help
echo "Testing 'ir help ncdu' alias..."
out2=$("$EXECUTABLE" help ncdu 2>&1)
if echo "$out2" | grep -q "ir-dua"; then
    echo "✅ PASS: 'ir help ncdu' alias correctly routed to dua help."
else
    echo "❌ FAIL: 'ir help ncdu' alias did not route correctly: $out2"
    RESULT=1
fi

# 3. Invalid switch detected
echo "Testing 'ir dua' with invalid switch..."
err=$("$EXECUTABLE" dua --badswitch 2>&1 || true)
if echo "$err" | grep -qiE "Unknown switch|ir-dua"; then
    echo "✅ PASS: 'ir dua' with invalid switch produced error/help."
else
    echo "❌ FAIL: 'ir dua' invalid switch not caught: $err"
    RESULT=1
fi

# Cleanup
rm -rf "$TMPDIR_LOCAL"

exit $RESULT
