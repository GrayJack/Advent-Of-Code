#!/bin/sh

YEAR=2018

if [ $# != 1 ]; then
    echo "Usage: $(basename "$0") <day-number>" >&2
    exit 1
fi
if [ ! -d .git ]; then
    echo "must be run from root of advent-of-code repository" >&2
    exit 1
fi

name="$(printf "day%02d" "$1")"
cargo new --bin "$YEAR/$name-1"
cargo new --bin "$YEAR/$name-2"
mkdir "$YEAR/$name-1/input"
mkdir "$YEAR/$name-2/input"
