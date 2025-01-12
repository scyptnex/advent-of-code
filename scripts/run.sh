#! /usr/bin/env bash

source "$(dirname "$0")/aoc.rc"
aoc_parse_year_day "$@"

pushd "${AOC_DIR}/${YEAR}"
./bin/compile "${DAY}"
./bin/run "${DAY}" < "${INPUT_PROD}" 
