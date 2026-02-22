#!/bin/bash

echo "Module                 Files    Code   Comment    Blank"
echo "----------------------------------------------------------------"

members=$(sed -n '/^\[workspace\]/,/^}/p' Cargo.toml | grep -o '"[^"]*"' | tr -d '"')

total_files=0
total_code=0
total_comment=0
total_blank=0

for dir in $members; do
    if [ -d "$dir/src" ]; then
        result=$(cloc "$dir/src" --quiet --json)
        
        files=$(echo "$result" | awk -F': ' '/"nFiles"/ {gsub(/[^0-9]/, "", $2); print $2; exit}')
        code=$(echo "$result" | awk -F': ' '/"code"/ && !/SUM/ {gsub(/[^0-9]/, "", $2); print $2; exit}')
        comment=$(echo "$result" | awk -F': ' '/"comment"/ && !/SUM/ {gsub(/[^0-9]/, "", $2); print $2; exit}')
        blank=$(echo "$result" | awk -F': ' '/"blank"/ && !/SUM/ {gsub(/[^0-9]/, "", $2); print $2; exit}')

        printf "%-22s %6s   %7s   %7s   %7s\n" "$dir" "$files" "$code" "$comment" "$blank"

        total_files=$((total_files + files))
        total_code=$((total_code + code))
        total_comment=$((total_comment + comment))
        total_blank=$((total_blank + blank))
    fi
done

echo "----------------------------------------------------------------"
printf "%-22s %6s   %7s   %7s   %7s\n" "Total" "$total_files" "$total_code" "$total_comment" "$total_blank"
