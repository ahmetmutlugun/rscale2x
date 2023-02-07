#!/bin/bash
# This is a simple test to see the real world performance of rust's scale2x vs the original C scale2x.
# There are too many factors to consider, so this is a rough comparison.
# I hope to include more detailed performance metrics in the future.

echo "Sleeping to cool down"
sleep 120
echo "Starting rust2x..."

START=$(date +%s)

for i in {1..500}
do
   ./target/release/rust_scale2x -i ./tests/duck.png -o output3.png
done

FINISH=$(date +%s)

echo "It takes $((FINISH - START)) seconds for rust2x to upscale duck 500 times."

#----------------------------------------
echo "Sleeping to cool down"
sleep 120
echo "Starting scale2x..."

START=$(date +%s)

for i in {1..500}
do
   scalex -k 2 ./tests/duck.png output.png
done

FINISH=$(date +%s)

echo "It takes $((FINISH - START)) seconds for scalex to upscale duck 500 times."

