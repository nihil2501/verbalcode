#!/bin/sh

# Override wasm32-unknown-unknown with default rustc target
cargo test --target=$(rustc -vV | awk '/host:/ {print $2}')