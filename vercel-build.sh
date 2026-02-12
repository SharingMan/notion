#!/bin/bash
set -e

echo "🦀 Installing Rust..."
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env

echo "📦 Installing Trunk..."
cargo install trunk --locked

echo "🎯 Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "🔨 Building project..."
trunk build --release

echo "✅ Build complete!"
ls -la dist/
