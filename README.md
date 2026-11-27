# Advent of Code

Solutions in Rust, every year in one Cargo workspace.

- `aoc/`: shared helpers (`aoc::*`) and the `new-day` and `solve` tools.
- `YEAR/`: one crate per event (`aocYEAR`), with one binary per day in `YEAR/src/bin/dayNN.rs`. Code shared by that year's days goes in `YEAR/src/lib.rs`.

The workspace targets the newest stable Rust: edition 2024, `rust-version = "1.99"`.
- `templates/day.rs`: the starting point for each day.

## Setup

Put the `session` cookie from adventofcode.com in `.env` (gitignored). `AOC_YEAR` is optional and sets the year used when only a day is given:

```
AOC_SESSION=...
AOC_YEAR=2026
```

## Daily loop

```sh
cargo new-day 2026 1   # create 2026/src/bin/day01.rs and fetch inputs/2026/day01.txt
cargo test -p aoc2026 --bin day01   # paste the example into EXAMPLE, set the expected answers
cargo solve 2026 1     # run both parts on the real input, in release mode, with timings
```

- The year is optional. Without it, the year is `AOC_YEAR`, or else the latest event.
- With no arguments during December, `new-day` and `solve` use today's puzzle.
- The first `new-day` for a year creates that year's crate with `cargo new`, which adds it to the workspace. Older events work the same way: `cargo new-day 2015 1`.
- Each step is skipped if its file already exists, so an input is downloaded only once.

Puzzle inputs live in `inputs/` and are not committed: the Advent of Code author asks that they not be published.

## Helpers (`aoc/src/lib.rs`)

- `read_input!()`: the input for the calling day, worked out from the crate and binary names
- `timed(label, || ...)`: run a part and print its answer and time
- `ints(s)`: every signed integer in a string
- `Grid`: parse a character grid, `get`/`set`/`find`/`positions`, `neighbours(p, &DIRS4 or &DIRS8)`
- `Pos = (row, col)`, `DIRS4`, `DIRS8`, `add`
