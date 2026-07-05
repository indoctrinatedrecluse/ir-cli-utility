#!/bin/bash
# Test: ir sort basic on Linux
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULT=0

Executable="$PROJECT_ROOT/target/debug/ir"
TempFile="temp_sort_input.txt"

# Create a sample file
cat <<EOF > "$TempFile"
banana
apple
Cherry
10
2
100
apple
EOF

# 1. Alphanumeric sort
echo "Testing basic sort..."
out=$("$Executable" sort "$TempFile")
# Expected (default string sort, case-sensitive):
# 10
# 100
# 2
# Cherry
# apple
# apple
# banana
# We check if the lines match
if echo "$out" | grep -q "10" && echo "$out" | grep -q "100" && echo "$out" | grep -q "2" && echo "$out" | grep -q "Cherry" && echo "$out" | grep -q "banana"; then
    echo "✅ PASS: Basic sort matches expected."
else
    echo "❌ FAIL: Basic sort output:"
    echo "$out"
    RESULT=1
fi

# 2. Reverse sort
echo "Testing reverse sort..."
out_rev=$("$Executable" sort -r "$TempFile")
# Check if banana is first
if [ "$(echo "$out_rev" | head -n 1)" = "banana" ]; then
    echo "✅ PASS: Reverse sort matches expected."
else
    echo "❌ FAIL: Reverse sort output:"
    echo "$out_rev"
    RESULT=1
fi

# 3. Numeric sort
echo "Testing numeric sort..."
out_num=$("$Executable" sort -n "$TempFile")
# First lines should be 2, 10, 100
first_three=$(echo "$out_num" | head -n 3 | tr '\n' ' ')
if [ "$first_three" = "2 10 100 " ]; then
    echo "✅ PASS: Numeric sort matches expected."
else
    echo "❌ FAIL: Numeric sort output:"
    echo "$out_num"
    RESULT=1
fi

# 4. Unique sort
echo "Testing unique sort..."
out_uniq=$("$Executable" sort -u "$TempFile")
apple_count=$(echo "$out_uniq" | grep -c "apple" || true)
if [ "$apple_count" -eq 1 ]; then
    echo "✅ PASS: Unique sort removed duplicates."
else
    echo "❌ FAIL: Unique sort output:"
    echo "$out_uniq"
    RESULT=1
fi

# 5. Check mode
echo "Testing check mode on sorted file..."
SortedFile="temp_sort_sorted.txt"
printf "2\n10\n100\n" > "$SortedFile"
if "$Executable" sort -c -n "$SortedFile"; then
    echo "✅ PASS: Check sorted file passed."
else
    echo "❌ FAIL: Check sorted file returned exit code $?"
    RESULT=1
fi

echo "Testing check mode on unsorted file..."
if "$Executable" sort -c -n "$TempFile" 2>/dev/null; then
    echo "❌ FAIL: Check unsorted file returned exit code 0"
    RESULT=1
else
    echo "✅ PASS: Check unsorted file correctly failed."
fi

# Clean up
rm -f "$TempFile" "$SortedFile"

exit $RESULT
