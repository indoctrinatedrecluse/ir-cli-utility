#!/usr/bin/env bash
# Test: ir browse — verify help routing, fm alias, and invalid switch handling.

set -e
echo "Building..."
cargo build --quiet --manifest-path "../../Cargo.toml"
EXECUTABLE="../../target/debug/ir"
RESULT=0

# 1. Help routing
echo "Testing 'ir help browse'..."
out=$("$EXECUTABLE" help browse 2>&1)
if echo "$out" | grep -q "ir-browse" && echo "$out" | grep -q "\[PATH\]" && echo "$out" | grep -qi "copy" && echo "$out" | grep -qi "delete"; then
    echo "✅ PASS: 'ir help browse' returned correct help text."
else
    echo "❌ FAIL: 'ir help browse' did not return expected output: $out"
    RESULT=1
fi

# 2. Alias 'fm' maps to browse help
echo "Testing 'ir help fm' alias..."
out2=$("$EXECUTABLE" help fm 2>&1)
if echo "$out2" | grep -q "ir-browse"; then
    echo "✅ PASS: 'ir help fm' alias correctly routed to browse help."
else
    echo "❌ FAIL: 'ir help fm' alias did not route correctly: $out2"
    RESULT=1
fi

# 3. Invalid switch detected
echo "Testing 'ir browse' with invalid switch..."
err=$("$EXECUTABLE" browse --badswitch 2>&1 || true)
if echo "$err" | grep -qiE "Unknown switch|ir-browse"; then
    echo "✅ PASS: 'ir browse' with invalid switch produced error/help."
else
    echo "❌ FAIL: 'ir browse' invalid switch not caught: $err"
    RESULT=1
fi

exit $RESULT
