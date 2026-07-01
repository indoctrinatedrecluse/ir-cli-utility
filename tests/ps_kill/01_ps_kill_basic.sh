#!/bin/bash
# Test: ps and kill actions on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: Basic process listing ---
echo "Testing basic ps command..."
PS_OUT=$($EXECUTABLE ps -n 5)
if [[ $PS_OUT == *PID*COMMAND* ]]; then
    echo "PASS: ps header is correct."
else
    echo "FAIL: ps output format mismatch. Output:"
    echo "$PS_OUT"
    exit 1
fi

# --- Test 2: Spawning process, filtering, and killing by name ---
echo "Spawning a temporary background sleep process..."
sleep 30 &
SLEEP_PID=$!
echo "Spawned sleep process with PID: $SLEEP_PID"

# Wait a brief moment
sleep 0.25

# Check process exists in ir ps
echo "Searching process using ir ps -f sleep..."
FILTERED_PS=$($EXECUTABLE ps -f sleep)
if [[ $FILTERED_PS == *sleep* ]]; then
    echo "PASS: Found spawned process in process list."
else
    echo "FAIL: Did not find sleep in process list. Output:"
    echo "$FILTERED_PS"
    kill -9 $SLEEP_PID 2>/dev/null
    exit 1
fi

# Terminate process using ir kill by name
echo "Terminating process using ir kill..."
$EXECUTABLE kill sleep -a
if [ $? -ne 0 ]; then
    echo "FAIL: ir kill command failed with exit code $?."
    kill -9 $SLEEP_PID 2>/dev/null
    exit 1
fi

# Verify process is terminated
sleep 0.25
FILTERED_PS_AFTER=$($EXECUTABLE ps -f sleep)
if [[ $FILTERED_PS_AFTER == *sleep* ]]; then
    echo "FAIL: Process still exists after kill. Output:"
    echo "$FILTERED_PS_AFTER"
    kill -9 $SLEEP_PID 2>/dev/null
    exit 1
else
    echo "PASS: Process was successfully terminated and is gone."
fi

# --- Test 3: Spawning process and killing by PID ---
echo "Spawning another temporary background sleep process..."
sleep 30 &
SLEEP_PID2=$!
echo "Spawned sleep process with PID: $SLEEP_PID2"

sleep 0.25

# Terminate process using ir kill by PID
echo "Terminating process by PID using ir kill..."
$EXECUTABLE kill $SLEEP_PID2
if [ $? -ne 0 ]; then
    echo "FAIL: ir kill <PID> failed with exit code $?."
    kill -9 $SLEEP_PID2 2>/dev/null
    exit 1
fi

# Verify process is terminated
sleep 0.25
FILTERED_PS_AFTER2=$($EXECUTABLE ps -f sleep)
if [[ $FILTERED_PS_AFTER2 == *sleep* ]]; then
    echo "FAIL: Process still exists after kill by PID. Output:"
    echo "$FILTERED_PS_AFTER2"
    kill -9 $SLEEP_PID2 2>/dev/null
    exit 1
else
    echo "PASS: Process was successfully terminated by PID."
fi

echo "ALL PS AND KILL TESTS PASSED"
exit 0
