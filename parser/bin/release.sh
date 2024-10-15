#! /bin/env bash

SCRIPT_PATH=$(realpath "$0")
RUST_PARSER_DIR=$(dirname $(dirname $SCRIPT_PATH))
ROOT_DIR=$(dirname $RUST_PARSER_DIR)
TEST_PROJECTS_DIR="$RUST_PARSER_DIR/tests/projects"
OUT_DIR="$RUST_PARSER_DIR/output"
NUMBER_OF_TEST_PROJECTS=$(ls $TEST_PROJECTS_DIR | wc -l)

rm -rf $OUT_DIR
# cargo build --release --quiet

# Capture the start time for the entire script
start_time=$(date +%s)

for folder in $(ls $TEST_PROJECTS_DIR); do
  # Capture the start time for each cargo run command
  folder_start_time=$(date +%s)

  $ROOT_DIR/target/release/parser --input "$TEST_PROJECTS_DIR/$folder" --output "$OUT_DIR/$folder" --stdout --json --cargo-toml --pretty $@
  # --stdout --json --cargo-toml $@

  if [ $? -ne 0 ]; then
    echo "Failed to run the parser for $folder"
    exit 1
  fi

  # Capture the end time for each cargo run command
  folder_end_time=$(date +%s)

  # Calculate and print the elapsed time for each cargo run command
  folder_elapsed_time=$(expr $folder_end_time - $folder_start_time)
  # echo "Time taken for $folder: $folder_elapsed_time seconds"
done

# Capture the end time for the entire script
end_time=$(date +%s)

# Calculate and print the total elapsed time
total_elapsed_time=$(expr $end_time - $start_time)
echo "Total time taken for $NUMBER_OF_TEST_PROJECTS projects with options $@: $total_elapsed_time seconds"
