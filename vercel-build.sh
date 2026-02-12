#!/bin/bash

# Vercel Build Script for Rust + WASM project
# This script installs Rust, builds the project with Trunk

set -e

echo "========================================="
echo "🚀 Starting Vercel Build"
echo "========================================="

# Set up Rust environment
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

# Check if Rust is already installed (cached)
if [ -f "$CARGO_HOME/bin/cargo" ]; then
    echo "✅ Rust found in cache"
else
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
        --default-toolchain stable \
        --profile minimal \
        --no-modify-path
fi

# Source cargo environment
source "$CARGO_HOME/env"

# Verify installation
echo ""
echo "📋 Rust Version:"
rustc --version
cargo --version
echo ""

# Install trunk if not present
if [ -f "$CARGO_HOME/bin/trunk" ]; then
    echo "✅ Trunk found in cache"
else
    echo "📦 Installing Trunk..."
    cargo install trunk --force
fi

# Add WASM target (idempotent)
echo "🎯 Adding WASM target..."
rustup target add wasm32-unknown-unknown || true

echo ""
echo "🔨 Building project with Trunk..."
echo ""

# Build the project with explicit public URL
trunk build --release --public-url ""

echo ""
echo "========================================="
echo "✅ Build Complete!"
echo "========================================="
echo ""
echo "📁 Output files:"
ls -lh dist/
echo ""
echo "🎉 Ready for deployment!"
