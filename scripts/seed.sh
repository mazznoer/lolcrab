#!/bin/bash

# Usage: ./seed.sh [seed]

BOLD="\033[1;37m"
RESET="\033[0m"

title () {
    printf "\n${BOLD}%b${RESET}\n\n" "$1"
}

lolcrab="$(dirname "$(realpath "$0")")/../target/debug/lolcrab"

# use 0 if seed not provided
seed=${1:-0}

text=$(fold -sw 56 lorem.txt | head -n 10)

title "seed: $seed"

for (( i=0; i<5; i=i+1 )); do
    $lolcrab --seed "$seed" <<< "$text"
    echo ""
done

title "without seed"

for (( i=0; i<5; i=i+1 )); do
    $lolcrab <<< "$text"
    echo ""
done
