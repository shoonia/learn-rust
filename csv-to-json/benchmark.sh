#!/bin/bash

# Ensure we have a file argument
if [ -z "$1" ]; then
echo "Usage: ./benchmark.sh <path_to_csv_file>"
exit 1
fi

echo "Building release binary...\n"
cargo build --release
echo "Build complete.\n"

echo "Running benchmark on $1..."
# usage of `time` ensures we measure execution without changing code
/usr/bin/time -l ./target/release/csv-to-json "$1"
