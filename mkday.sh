#!/bin/sh

if [ -z "$1" ]; then
  echo "Missing day #"
  exit 1
fi

DAY=$1

cargo new day$DAY
touch private/inputs/in_$DAY.txt
touch private/inputs/sample_$DAY.txt
cp boiler/tests.txt private/inputs/tests_$DAY.txt
touch private/puzzles/$DAY.txt


cd day$DAY

ln -s ../private/inputs/in_$DAY.txt input.txt
ln -s ../private/inputs/sample_$DAY.txt sample.txt
ln -s ../private/inputs/tests_$DAY.txt tests.txt
ln -s ../private/puzzles/$DAY.txt readme.txt

cargo add --path ../aoc_tools/
cargo add anyhow
cargo add itertools
cargo add rstest
cp ../boiler/main.rs src/