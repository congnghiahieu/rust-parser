#! /bin/env bash

SCRIPT_PATH=$(realpath "$0")
ROOT_DIR=$(dirname $(dirname $SCRIPT_PATH))
TEMP_DIR="$ROOT_DIR/tests/temp"
OUT_DIR="$ROOT_DIR/output"

# rm -rf "$OUT_DIR"

cargo run --release -- --input "$TEMP_DIR" --output "$OUT_DIR/temp" --stdout --json --cargo-toml --pretty
