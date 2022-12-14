#! /bin/bash

# Advent of Code helper commands

ACTION="$1"
DAY="$2"

TEMPLATE="templates/day.template.rs"
INPUT_TEMPLATE="templates/day_input.template.rs"

#EXTERNS="--extern ndarray=deps/libndarray-4276025192ebc488.rlib"

if [[ ${#DAY} == 1 ]]; then
	DAY="0${DAY}"
fi

DAY_NO_LEADING=$(($DAY))

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

		sed -i -s -r "s/\\{\\{day\\}\\}/${DAY}/" "puzzles/day${DAY}.rs"
		sed -i -s -r "s/\\{\\{topic\\}\\}/${TOPIC}/" "puzzles/day${DAY}.rs"
		sed -i -s -r "s/(\\s+)\\/\\/ \\{\\{new_day\\}\\}/\\1${DAY_NO_LEADING} => puzzles::day${DAY}::main(),\\n\\0/" "main.rs"

		echo "pub mod day${DAY};" >> puzzles.rs

		echo "Created files for day ${DAY} - ${TOPIC}."
	fi

elif [[ "$ACTION" == "run" ]]; then
	cargo build && ./target/debug/aoc2022 ${DAY}
else
	echo "Unknown action ('$ACTION')."
	exit 1
fi
