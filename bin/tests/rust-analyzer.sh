#! /bin/env bash

SCRIPT_PATH=$(realpath $0)
SCRIPT_DIR=$(dirname $SCRIPT_PATH)
SRC_FILE_DIR=$SCRIPT_DIR/../src
SRC_FILE_PATH=$SRC_FILE_DIR/main.rs
CARGO_TOML_DIR=$SCRIPT_DIR/../
CARGO_TOML_PATH=$CARGO_TOML_DIR/Cargo.toml

cat $SRC_FILE_PATH | rust-analyzer parse
# cat $SRC_FILE_PATH | rust-analyzer symbols
# cat $SRC_FILE_PATH | rust-analyzer highlight --rainbow
# rust-analyzer analysis-stats $CARGO_TOML_DIR
# rust-analyzer diagnostics $CARGO_TOML_DIR
# rust-analyzer lsif $CARGO_TOML_DIR --exclude-vendored-libraries >lsif.json
# rust-analyzer scip $CARGO_TOML_DIR --exclude-vendored-libraries
