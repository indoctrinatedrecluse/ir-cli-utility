#!/usr/bin/env bash
# Test: ir edit — help routing, alias, error handling for missing filename and invalid switch.
# Run from project root: bash tests/edit/01_edit_help.sh
#
# NOTE: Interactive editing (keystroke simulation) cannot be automated in a non-TTY
# environment. These tests cover all automatable surface: routing, aliases, and
# CLI error guards. Actual editing is validated manually via 'ir edit <file>'.

set -e
echo "Building..."
cargo build --quiet
EXECUTABLE="./target/debug/ir"
RESULT=0

# 1. Help text is correct
echo "Testing 'ir help edit'..."
out=$("$EXECUTABLE" help edit 2>&1)
if echo "$out" | grep -q "ir-edit" && \
   echo "$out" | grep -q "Ctrl+S" && \
   echo "$out" | grep -q "Ctrl+Q" && \
   echo "$out" | grep -q "Arrow keys"; then
    echo "✅ PASS: 'ir help edit' returned correct help text."
else
    echo "❌ FAIL: 'ir help edit' unexpected output: $out"
    RESULT=1
fi

# 2. Alias 'ed' routes to edit help
echo "Testing 'ir help ed' alias..."
out2=$("$EXECUTABLE" help ed 2>&1)
if echo "$out2" | grep -q "ir-edit"; then
    echo "✅ PASS: 'ir help ed' alias correctly routed to edit help."
else
    echo "❌ FAIL: 'ir help ed' alias did not route correctly: $out2"
    RESULT=1
fi

# 3. Calling 'ir edit' without a filename produces an error
echo "Testing 'ir edit' with no filename..."
err=$("$EXECUTABLE" edit 2>&1 || true)
if echo "$err" | grep -qiE "requires a filename|ir-edit"; then
    echo "✅ PASS: 'ir edit' (no filename) correctly showed error/help."
else
    echo "❌ FAIL: 'ir edit' (no filename) did not error correctly: $err"
    RESULT=1
fi

# 4. Unknown switch produces an error
echo "Testing 'ir edit' with an unknown switch..."
err2=$("$EXECUTABLE" edit --badswitch 2>&1 || true)
if echo "$err2" | grep -qiE "Unknown switch|ir-edit"; then
    echo "✅ PASS: 'ir edit --badswitch' correctly showed error/help."
else
    echo "❌ FAIL: 'ir edit --badswitch' did not error correctly: $err2"
    RESULT=1
fi

# 5. Passing a directory path as filename should error
echo "Testing 'ir edit' on a directory path..."
dirErr=$("$EXECUTABLE" edit src 2>&1 || true)
if echo "$dirErr" | grep -qi "directory"; then
    echo "✅ PASS: 'ir edit src' correctly rejected a directory."
else
    echo "❌ FAIL: 'ir edit src' did not reject directory: $dirErr"
    RESULT=1
fi

exit $RESULT
