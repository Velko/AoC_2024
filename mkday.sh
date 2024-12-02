#!/bin/sh

if [ -z "$1" ]; then
  echo "Missing day #"
  exit 1
fi

DAY=$1

cargo new day$DAY
touch inputs/in_$DAY.txt
touch puzzles/$DAY.txt

cd day$DAY

ln -s ../inputs/in_$DAY.txt input.txt
ln -s ../puzzles/$DAY.txt readme.txt

cargo add --path ../aoc_tools/
cargo add anyhow
cargo add itertools
cp ../boiler/main.rs src/