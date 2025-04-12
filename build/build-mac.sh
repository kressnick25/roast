#!/usr/bin/env bash

TARGET="aarch64-apple-darwin"
PACKAGE="roast-macOS-arm64.tar.gz"

echo "Building"
cargo build --locked --release --target $TARGET

echo "Stripping"
strip target/$TARGET/release/roast

echo "Packaging"
tar czvf $PACKAGE -C target/$TARGET/release .

# Generate checksum
HASH=$(shasum -a 256 $PACKAGE)

echo "---- SHA256 hash of $PACKAGE ----"
echo $HASH

echo $HASH > $PACKAGE.sha256
