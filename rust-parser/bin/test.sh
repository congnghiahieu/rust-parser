#! /bin/env bash

SCRIPT_PATH=$(realpath "$0")
ROOT_DIR=$(dirname $(dirname $SCRIPT_PATH))
TEST_PROJECTS_DIR="$ROOT_DIR/tests/projects"
EXAMPLES_DIR="$ROOT_DIR/tests/examples"
OUT_DIR="$ROOT_DIR/output"
NUMBER_OF_TEST_PROJECTS=$(ls $TEST_PROJECTS_DIR | wc -l)

cargo run --release -- --input "$EXAMPLES_DIR" --output "$OUT_DIR/examples" $@
