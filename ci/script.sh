#!/bin/sh

set -ex

cargo build  --features="$FEATURES"
cargo test   --features="$FEATURES"
cargo clippy --features="$FEATURES" -- -D clippy::all
cargo run    --features="$FEATURES" -- --help
cargo fmt                           -- --check
