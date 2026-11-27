# Workflow

## Setup

Settings live in `.env` at the workspace root. It is gitignored.

```
AOC_SESSION=<the session cookie from adventofcode.com>
AOC_YEAR=2026
```

To get the cookie:

1. Log in to adventofcode.com.
2. Open the browser dev tools.
3. Copy the value of the `session` cookie, from Storage in Firefox or Application in Chrome.

The cookie lasts about a month. When fetching or submitting fails with an HTTP error, get a new one.

## Commands

| Command | What it does |
| --- | --- |
| `cargo new-day [year] [day]` | Create `YEAR/src/bin/dayNN.rs` with the example filled in, and fetch `inputs/YEAR/dayNN.txt` |
| `cargo test -p aoc2026 --bin day01` | Run that day's example tests |
| `cargo solve [year] [day]` | Run both parts on the real input, in release mode, with timings |
| `cargo submit [year] [day] part answer` | Send an answer and print the site's reply |
| `cargo watch -x 'test -p aoc2026 --bin day01'` | Re-run the tests on every save (needs `cargo install cargo-watch`) |

- With no year, the tools use `AOC_YEAR`, or else the latest event.
- With no year and no day during December, they use today's puzzle.
- `new-day` skips any file that already exists, so running it twice is safe and the input is downloaded once.
- Puzzles unlock at 00:00 EST (UTC-5), which is 06:00 in Sweden.
- From 2025 on, an event has 12 days, not 25.

## A day, step by step

1. `cargo new-day` when the puzzle unlocks.
2. Read the puzzle. Check that `EXAMPLE` in the tests matches the example in the text. Sometimes the right example is a later `<pre>` block.
3. Put the expected example answer in `part1_example` and write `part1` until the test passes.
4. `cargo solve`, then `cargo submit 2026 1 1 <answer>`.
5. Part 2 appears on the puzzle page. Repeat steps 3 and 4 with `part2`.

## The day file

```rust
use aoc::*;

fn part1(input: &str) -> i64 { ... }
fn part2(input: &str) -> i64 { ... }

fn main() {
    let input = read_input!();
    timed("Part 1", || part1(&input));
    timed("Part 2", || part2(&input));
}
```

- Change the return type to `u64`, `usize` or `String` when the answer needs it. `timed` takes anything that implements `Display`.
- If parsing is shared and slow, parse once in `main` and pass the parsed data to both parts.
- For a puzzle with several examples, add more `const`s and tests.
- Put code that several days of one year share in `YEAR/src/lib.rs`, and use it as `aoc2026::...`. Put code useful across years in `aoc/src/lib.rs`.

## Tips

- Read the whole puzzle, then read the last paragraph again. That is where the question is.
- Get the example test passing before running on the real input.
- If part 1 is a simulation, expect part 2 to ask for the same thing a huge number of times. Look for a cycle, a count per kind, or maths. See [Algorithms](algorithms/README.md).
- Look at the real input. It often has structure the text does not mention, such as one big loop, a grid with an empty middle row, or numbers that are all prime.
- Use `i64` or `u64` by default. Release builds wrap on overflow without a warning; debug builds (the tests) panic instead.
- Slow? Run in release with `cargo solve`. Swap `HashMap` for `FxHashMap`, or for a `Vec` indexed by position. Avoid cloning big states.
- `dbg!(x)` prints a value with its file and line, and returns it, so it can wrap any expression.
- After a wrong answer, the site makes you wait before the next try, and the wait grows each time.

## References

- [Advent of Code](https://adventofcode.com/) and its [about page](https://adventofcode.com/about), including the automation rules this repo follows: inputs are not committed, and each input is downloaded only once.
- [r/adventofcode](https://www.reddit.com/r/adventofcode/): daily solution threads, worth reading after solving.
- [Rust standard library docs](https://doc.rust-lang.org/std/).
- [`itertools` docs](https://docs.rs/itertools/latest/itertools/trait.Itertools.html).
- [Red Blob Games](https://www.redblobgames.com/): clear, interactive guides to grids and pathfinding.
