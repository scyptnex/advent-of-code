#!/usr/bin/env bash

set -eux

DAY=`date +%d`
YEAR=2023
PROJECT_DIR=$(dirname `readlink -f $0`)

xdg-open https://adventofcode.com/$YEAR/day/${DAY#0}
xdg-open https://adventofcode.com/$YEAR/day/${DAY#0}/input 

TODAY=$PROJECT_DIR/src/bin/p${DAY}.rs
[ -f $TODAY ] || cp $PROJECT_DIR/src/bin/p00.rs $TODAY

vim $TODAY
