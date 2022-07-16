#!/bin/sh
# Lint the repo
echo "Running cargo fmt"
cargo fmt --all -- --check

# Quickly checks your code to make sure it compiles 
# but doesn’t produce an executable
echo "Running cargo check ..."
cargo check

# Creates build executables.
echo "Running cargo build ..."
cargo build

echo "Running cargo test ..."
cargo test
