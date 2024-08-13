#!/bin/bash

# Usage: ./random_colors.sh [i]

lolcrab="$(dirname "$(realpath "$0")")/../target/debug/lolcrab"

if [[ "$1" == "i" ]]; then
    invert="--invert"
fi

text=$(fold -sw 56 lorem.txt | head -n 23)

for (( i=0; i<10; i=i+1 )); do
    $lolcrab --random-colors 5 $invert <<< "$text"
    echo ""
done
