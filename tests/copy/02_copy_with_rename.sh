#!/bin/bash

# Test: Copy a single file with the --rename switch.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_copy_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

touch "source.log"
mkdir "logs_backup"

# --- Test ---
echo "Running test: ir copy source.log logs_backup --rename backup.log"
"$EXECUTABLE" copy "source.log" "logs_backup" --rename "backup.log"

# --- Verification ---
if [ -f "logs_backup/backup.log" ]; then
    echo "✅ PASS: File was copied and renamed successfully."
    RESULT=0
else
    echo "❌ FAIL: Renamed file not found in the destination directory."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
