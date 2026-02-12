#!/bin/bash
set -e

echo "🔧 Setting up environment..."
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

echo "🦀 Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
source "$CARGO_HOME/env"

echo "✅ Rust version:"
rustc --version

echo "📦 Installing Trunk..."
cargo install trunk --force

echo "🎯 Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "🔨 Building with Trunk..."
trunk build --release --public-url "/"

echo "✅ Build complete! Contents of dist/:"
ls -la dist/
