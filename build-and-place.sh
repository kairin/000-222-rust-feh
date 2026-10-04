#!/usr/bin/env bash
# Helper: build rust-feh and place a copy in the project root (for the plan's "output to root" step)
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

echo "✅ Binary placed at ./rust-feh (ready for later spec-kit execution)"
ls -l ./rust-feh
