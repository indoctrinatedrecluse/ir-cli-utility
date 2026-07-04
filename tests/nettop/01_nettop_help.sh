#!/usr/bin/env bash
# Test: ir nettop — verify help routing and alias.

set -e
echo "Building..."
cargo build --quiet --manifest-path "../../Cargo.toml"
EXECUTABLE="../../target/debug/ir"
RESULT=0

# 1. Help routing
echo "Testing 'ir help nettop'..."
out=$("$EXECUTABLE" help nettop 2>&1)
if echo "$out" | grep -q "ir-nettop" && echo "$out" | grep -q "\-\-delay"; then
    echo "✅ PASS: 'ir help nettop' returned correct help text."
else
    echo "❌ FAIL: 'ir help nettop' did not return expected output: $out"
    RESULT=1
fi

# 2. Alias 'ntop' maps to nettop help
echo "Testing 'ir help ntop' alias..."
out2=$("$EXECUTABLE" help ntop 2>&1)
if echo "$out2" | grep -q "ir-nettop"; then
    echo "✅ PASS: 'ir help ntop' alias correctly routed to nettop help."
else
    echo "❌ FAIL: 'ir help ntop' alias did not route correctly: $out2"
    RESULT=1
fi

# 3. Invalid switch detected
echo "Testing 'ir nettop' with invalid switch..."
err=$("$EXECUTABLE" nettop --badswitch 2>&1 || true)
if echo "$err" | grep -qiE "Unknown switch|ir-nettop"; then
    echo "✅ PASS: 'ir nettop' with invalid switch produced error/help."
else
    echo "❌ FAIL: 'ir nettop' invalid switch not caught: $err"
    RESULT=1
fi

exit $RESULT
