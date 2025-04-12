#!/usr/bin/env bash

TARGET='x86_64-unknown-linux-gnu'
PACKAGE='roast-Linux-x86_64.tar.gz'

echo "Building"
cargo build --locked --release --target $TARGET

echo "Stripping"
strip target/$TARGET/release/roast

echo "Packaging"
tar czvf $PACKAGE -C target/$TARGET/release roast

# Generate checksum
HASH=$(sha256sum $PACKAGE)

echo "---- SHA256 hash of $PACKAGE ----"
echo $HASH

echo $HASH > $PACKAGE.sha256
