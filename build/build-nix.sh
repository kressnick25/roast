#!/usr/bin/env bash

echo "Building"
cargo build --locked --release --target x86_64-unknown-linux-gnu

echo "Stripping"
strip target/x86_64-unknown-linux-gnu/release/roast

echo "Packaging"
tar czvf roast-Linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release .
