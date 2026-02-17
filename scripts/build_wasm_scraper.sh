#!/bin/bash
set -e

echo "🚀 Building WASM Scraper Module..."
cd src/wasm_scraper

# Ensure target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build
cargo build --target wasm32-unknown-unknown --release

echo "✅ WASM build complete at src/wasm_scraper/target/wasm32-unknown-unknown/release/nuclear_wasm_scraper.wasm"
