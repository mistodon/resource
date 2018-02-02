#!/bin/sh

set -e

echo "\033[1;33mTesting on debug with default features\033[0m"
cargo test

echo "\033[1;33mTesting on release with default features\033[0m"
cargo test --release

echo "\033[1;33mTesting on debug with force-static\033[0m"
cargo test --features force-static

echo "\033[1;33mTesting on release with force-dynamic\033[0m"
cargo test --release --features force-dynamic

