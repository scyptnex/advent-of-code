#! /usr/bin/env bash

source "$(dirname "$0")/aoc.rc"
aoc_parse_year_day "$@"

mkdir -p "${INPUT_DIR}"
aocd "${DAY}" "20${YEAR}" > "${INPUT_PROD}"
aocd -e reference "${DAY}" "20${YEAR}" > "${INPUT_TEST}"
aocd -e simple "${DAY}" "20${YEAR}" >> "${INPUT_TEST}"
