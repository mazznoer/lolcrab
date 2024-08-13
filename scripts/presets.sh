#!/bin/bash

# Usage: ./presets.sh [i]

BOLD="\033[1;37m"
RESET="\033[0m"

title () {
    printf "\n${BOLD}%b${RESET}\n\n" "$1"
}

presets=(
    cividis
    cool
    cubehelix
    fruits
    inferno
    magma
    plasma
    rainbow
    rd-yl-gn
    sinebow
    spectral
    turbo
    viridis
    warm
)

lolcrab="$(dirname "$(realpath "$0")")/../target/debug/lolcrab"

if [[ "$1" == "i" ]]; then
    invert="--invert"
fi

text=$(fold -sw 56 lorem.txt | head -n 23)

for str in "${presets[@]}"; do
    title "$str"
    $lolcrab --gradient "$str" $invert <<< "$text"
done
