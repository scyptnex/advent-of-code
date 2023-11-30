#!/usr/bin/env bash

set -eux

DAY=`date +%d`
YEAR=2023
PROJECT_DIR=$(dirname `readlink -f $0`)

URL=https://adventofcode.com/$YEAR/day/${DAY#0}

xdg-open $URL
xdg-open $URL/input

TODAY=$PROJECT_DIR/src/bin/p${DAY}.rs
[ -f $TODAY ] || cp $(dirname $TODAY)/p00.rs $TODAY

vim $TODAY
