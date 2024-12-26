#!/usr/bin/env bash

set -eux

BINARY=p${1:-`date +%d`}

cargo test --bin $BINARY
cargo run --bin $BINARY
