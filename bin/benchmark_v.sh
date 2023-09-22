#!/bin/bash

files=("CMT1" "CMT2" "CMT3" "CMT4" "CMT5" "CMT11" "CMT12")





for file_name in "${files[@]}"; do
    file_path="./src/res/CMT/$file_name.vrp" 
    # Use the string in a command (for example, echo)
    ./target/release/pipeline solve "$file_path" tsp qb-solv -d "./.vrp/$file_name/qbsolv_new" > "./.vrp/$file_name/qbsolv_new.txt"
    # You can replace the echo command with any other command that uses the string
done