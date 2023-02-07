#!/bin/bash
# This is a simple test to see the real world performance of rust's scale2x vs the original C scale2x.
# There are too many factors to consider, so this is a rough comparison.
# I hope to include more detailed performance metrics in the future.

echo "Sleeping to cool down"
sleep 1
echo "Starting rust2x..."

START=$(date +%s)

for i in {1..100}
do
   ../target/release/scale2x -i grid.png -o output2.png
   ../target/release/scale2x -i tree.png -o output2.png
done

FINISH=$(date +%s)

echo "It takes $((FINISH - START)) seconds for rust2x to upscale tree+grid 100 times."

#----------------------------------------
echo "Sleeping to cool down"
sleep 1
echo "Starting scale2x..."

START=$(date +%s)

for i in {1..100}
do
   scalex -k 2 grid.png output.png
   scalex -k 2 tree.png output.png
done

FINISH=$(date +%s)

echo "It takes $((FINISH - START)) seconds for scalex to upscale tree+grid 100 times."

rm output.png
rm output2.png