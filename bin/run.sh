#! /bin/env bash

CURRENT_DIR=$(dirname $(realpath --logical $0))
OUT_DIR=$CURRENT_DIR/../out

mkdir -p $OUT_DIR
cargo run
