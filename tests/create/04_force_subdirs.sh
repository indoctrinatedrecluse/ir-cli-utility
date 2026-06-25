#!/bin/bash

# Test: -p switch creates parent directories.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_create_04"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# --- Test ---
echo "Running test: ir create -p new/nested/directory"
"$EXECUTABLE" create -p "new/nested/directory"

# --- Verification ---
if [ -d "new/nested/directory" ]; then
    echo "✅ PASS: Nested directory structure was created successfully."
    RESULT=0
else
    echo "❌ FAIL: Nested directory was not created."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
