#!/bin/bash
set -e

echo "🦀 Installing Rust..."
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
source "$CARGO_HOME/env"

echo "✅ Rust installed:"
rustc --version
cargo --version

echo "📦 Installing Trunk..."
cargo install trunk --locked

echo "🎯 Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "🔨 Building project..."
trunk build --release

echo "✅ Build complete!"
ls -la dist/
