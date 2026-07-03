#!/bin/bash
# Test: Simple df
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

echo "Running test: ir df"
output=$($Executable df)
echo "$output"

if echo "$output" | grep -q "Filesystem" && echo "$output" | grep -q "Size"; then
    echo "✅ PASS: 'ir df' output header verified."
    exit 0
else
    echo "❌ FAIL: 'ir df' output headers missing."
    exit 1
fi
