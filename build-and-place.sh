#!/usr/bin/env bash
# Helper: build rust-feh in release mode and copy the binary to the project root
# Usage: ./build-and-place.sh
set -euo pipefail

if ! cargo build --release; then
    echo "Build failed. Make sure Rust is installed (rustup) and you're in the right dir."
    exit 1
fi

if ! cp target/release/rust-feh ./rust-feh; then
    echo "Copy failed"
    exit 1
fi

echo "✅ Binary placed at ./rust-feh"
ls -l ./rust-feh
