#! /bin/env bash

SCRIPT_PATH=$(realpath "$0")
ROOT_DIR=$(dirname $(dirname $SCRIPT_PATH))
TEST_PROJECTS_DIR="$ROOT_DIR/tests/projects"
OUT_DIR="$ROOT_DIR/output"
NUMBER_OF_TEST_PROJECTS=$(ls $TEST_PROJECTS_DIR | wc -l)

rm -rf $OUT_DIR

# Capture the start time for the entire script
start_time=$(date +%s)

for folder in $(ls $TEST_PROJECTS_DIR); do
  # Capture the start time for each cargo run command
  folder_start_time=$(date +%s)

  cargo run --release -- --input "$TEST_PROJECTS_DIR/$folder" --output "$OUT_DIR/$folder" $@

  # Capture the end time for each cargo run command
  folder_end_time=$(date +%s)

  # Calculate and print the elapsed time for each cargo run command
  folder_elapsed_time=$(expr $folder_end_time - $folder_start_time)
  echo "Time taken for $folder: $folder_elapsed_time seconds"
done

# Capture the end time for the entire script
end_time=$(date +%s)

# Calculate and print the total elapsed time
total_elapsed_time=$(expr $end_time - $start_time)
echo "Total time taken for $NUMBER_OF_TEST_PROJECTS projects with options $@: $total_elapsed_time seconds"
