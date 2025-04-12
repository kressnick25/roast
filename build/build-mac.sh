#!/usr/bin/env bash

echo "Building"
cargo build --locked --release --target aarch64-apple-darwin

echo "Stripping"
strip target/aarch64-apple-darwin/release/roast

echo "Packaging"
tar czvf roast-macOS-arm64.tar.gz -C target/aarch64-apple-darwin/release .
