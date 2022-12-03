#! /bin/bash

# Advent of Code helper commands

ACTION="$1"
DAY="$2"

TEMPLATE="templates/day.template.rs"
INPUT_TEMPLATE="templates/day_input.template.rs"

#EXTERNS="--extern ndarray=deps/libndarray-4276025192ebc488.rlib"

if [[ "$ACTION" == "create" ]]; then
	TOPIC="$3"
	if [[ -z "$TOPIC" ]]; then
		echo "No topic provided."
		exit 1
	fi
	if [ -f "puzzles/day${DAY}.rs" ]; then
		echo "Day already exists."
		exit 1
	else
		cp $TEMPLATE "puzzles/day${DAY}.rs"
		cp $INPUT_TEMPLATE "puzzles/day${DAY}_input.rs"

		sed -i -s -r "s/\\{\\{day\\}\\}/${DAY}/" "puzzles/day${DAY}.rs" "puzzles/day${DAY}_input.rs"
		sed -i -s -r "s/\\{\\{topic\\}\\}/${TOPIC}/" "puzzles/day${DAY}.rs" "puzzles/day${DAY}_input.rs"

		echo "pub mod day${DAY};" >> puzzles.rs
		echo "pub mod day${DAY}_input;" >> puzzles.rs
		echo "" >> puzzles.rs

		echo "Created files for day ${DAY} - ${TOPIC}."
	fi

elif [[ "$ACTION" == "run" ]]; then
	cargo build && ./target/debug/aoc2022 ${DAY}
else
	echo "Unknown action ('$ACTION')."
	exit 1
fi
