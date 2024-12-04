#! /bin/env bash

SCRIPT_PATH=$(realpath "$0")
ROOT_DIR=$(dirname $(dirname $SCRIPT_PATH))
EXAMPLES_DIR="$ROOT_DIR/tests/examples"
OUT_DIR="$ROOT_DIR/output"

# rm -rf "$OUT_DIR"

cargo run --release -- --input "$EXAMPLES_DIR" --output "$OUT_DIR/examples" --stdout --json --cargo-toml --pretty

if [ $? -ne 0 ]; then
  echo "Failed to run the parser for $folder"
  exit 1
fi
