#!/bin/bash
set -e

# Check if chpl is installed
if ! command -v chpl &> /dev/null; then
    echo "❌ Chapel compiler (chpl) not found. Chapel backend is required."
    exit 1
else
    echo "🚀 Compiling Chapel OSINT Library..."
    mkdir -p ffi/chapel

    # Compile to shared library
    chpl --library --dynamic --fast ffi/chapel/osint_lib.chpl -o ffi/chapel/libchapel_osint

    echo "✅ Chapel library built at ffi/chapel/libchapel_osint.so"
fi
