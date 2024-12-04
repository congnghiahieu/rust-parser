#! /bin/env bash

SCRIPT_PATH=$(realpath "$0")
ROOT_DIR=$(dirname $(dirname $SCRIPT_PATH))
BUILD_DIR="$ROOT_DIR/build"

cargo build --release
mkdir -p $BUILD_DIR
cp $ROOT_DIR/target/release/parser $BUILD_DIR/rust-parser
