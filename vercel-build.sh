#!/bin/bash

# Vercel Build Script for Rust + WASM (Yew) project

set -e

echo "========================================="
echo "🚀 Starting Vercel Build"
echo "========================================="

# Set up Rust environment
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

# Install Rust if not cached
if [ -f "$CARGO_HOME/bin/cargo" ]; then
    echo "✅ Rust found in cache"
else
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
        --default-toolchain stable \
        --profile minimal
fi

# Source cargo
source "$CARGO_HOME/env"

# Show versions
echo ""
echo "📋 Versions:"
rustc --version
cargo --version

# Install trunk
if [ -f "$CARGO_HOME/bin/trunk" ]; then
    echo "✅ Trunk found, updating..."
    cargo install trunk --force
else
    echo "📦 Installing Trunk..."
    cargo install trunk --locked
fi

# Add WASM target
echo "🎯 Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo ""
echo "🔨 Building with Trunk..."
echo ""

# Clean previous build
rm -rf dist/

# Build with Trunk - using relative paths for Vercel
trunk build --release

echo ""
echo "========================================="
echo "📁 Build output:"
echo "========================================="
ls -la dist/

echo ""
echo "🔍 Checking generated HTML:"
head -50 dist/index.html

echo ""
echo "✅ Build complete!"
