#!/bin/bash
cargo build -r
cp ../../target/release/aiken ~/.local/bin/cargo-aiken
cargo clean -r
