#! /usr/bin/env bash

source "$(dirname "$0")/aoc.rc"

if [ "$#" != 2 ]; then
    echo "usage: $0 <YEAR> <DAY>"
    echo
    echo "For example $> $0 $(date +"%y %d")"
    exit 42
fi

# Day and year as 2-digit numbers (zero padded)
YEAR="${1:(-2)}"
DAY="0$2"
DAY="${DAY:(-2)}"

INPUT_DIR="${AOC_DIR}/${YEAR}/input"
mkdir -p "${INPUT_DIR}"
aocd "${DAY}" "20${YEAR}" > "${INPUT_DIR}/d${DAY}"
aocd -e reference "${DAY}" "20${YEAR}" > "${INPUT_DIR}/t${DAY}"
aocd -e simple "${DAY}" "20${YEAR}" >> "${INPUT_DIR}/t${DAY}"
