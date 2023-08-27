#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 [file_path]"
  exit 1
fi

file_path="$1"
file_name=$(basename "$file_path" | cut -d. -f1)

./target/release/pipeline solve "$file_path" knn lkh -n 5 -d "./.vrp/$file_name/min_c" > "./.vrp/$file_name/min_c.txt"
echo "Solved LKH min cluster [5]"

./target/release/pipeline solve "$file_path" knn lkh -n 10 -d "./.vrp/$file_name/fixed_c" > "./.vrp/$file_name/fixed_c.txt"
echo "Solved LKH fixed 10 clusters"

./target/release/pipeline solve "$file_path" knn lkh -n 4 -d "./.vrp/$file_name/fixed_c_size" > "./.vrp/$file_name/fixed_c_size.txt"
echo "Solved LKH ~15 cluster size [4]"

./target/release/pipeline solve "$file_path" tsp qb-solv -d "./.vrp/$file_name/qbsolv" > "./.vrp/$file_name/qbsolv.txt"
echo "Solved qbsolv"

./target/release/pipeline solve "$file_path" tsp leap-hybrid -d "./.vrp/$file_name/leaphybrid" > "./.vrp/$file_name/leaphybrid.txt"
echo "Solved leaphybrid"

./target/release/pipeline solve "$file_path" knn direct -n 7 -d "./.vrp/$file_name/direct" > "./.vrp/$file_name/direct.txt"
echo "Solved direct"

./target/release/pipeline solve "$file_path" tsp simulated -d "./.vrp/$file_name/simulated" > "./.vrp/$file_name/simulated.txt"
echo "Solved simulated"

echo "Finished"