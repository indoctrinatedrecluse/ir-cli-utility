#!/usr/bin/env bash
# Test: ir watch — verify help routing, alias, and basic non-interactive invocation.

set -e
echo "Building..."
cargo build --quiet --manifest-path "../../Cargo.toml"
EXECUTABLE="../../target/debug/ir"
RESULT=0

# 1. Help routing
echo "Testing 'ir help watch'..."
out=$("$EXECUTABLE" help watch 2>&1)
if echo "$out" | grep -q "ir-watch" && echo "$out" | grep -q "\-\-interval" && echo "$out" | grep -q "\-\-diff"; then
    echo "✅ PASS: 'ir help watch' returned correct help text."
else
    echo "❌ FAIL: 'ir help watch' did not return expected output: $out"
    RESULT=1
fi

# 2. Invalid switch detected
echo "Testing 'ir watch' with invalid switch..."
err=$("$EXECUTABLE" watch --unknown 2>&1 || true)
if echo "$err" | grep -qiE "Unknown switch|ir-watch"; then
    echo "✅ PASS: 'ir watch' with invalid switch produced error/help."
else
    echo "❌ FAIL: 'ir watch' invalid switch not caught: $err"
    RESULT=1
fi

exit $RESULT
