# Helpers

Everything here comes in with `use aoc::*;`. The source is in `aoc/src/lib.rs` and `aoc/src/search.rs`.

## Input and timing

| Item | Use |
| --- | --- |
| `read_input!()` | The input for the calling day, as a `String`. Works out the year and day from the crate and binary names |
| `timed(label, \|\| ...)` | Run a part, then print its answer and how long it took |
| `ints(s) -> Vec<i64>` | Every signed integer in a string, in order |

`ints` ignores all other characters, so it handles most input lines on its own:

```rust
ints("x=-3, y=14")            // [-3, 14]
ints("move 3 from 1 to 2")    // [3, 1, 2]
ints("2-7")                   // [2, -7]: a dash before a digit counts as a minus sign
```

The last case matters for ranges like `2-7`. Use `split('-')` there instead.

## Positions and directions

| Item | Use |
| --- | --- |
| `Pos = (i32, i32)` | `(row, col)`. Row 0 is the top. Signed, so stepping off the edge is safe to compute |
| `DIRS4` | Up, right, down, left, in that order |
| `DIRS8` | The 8 directions, clockwise from up |
| `add(a, b)` | `a + b` |
| `turn_right(d)`, `turn_left(d)` | Turn a direction by 90 degrees |
| `manhattan(a, b)` | The number of straight steps between two positions |

Because `DIRS4` is in clockwise order, you can also keep a direction as an index: `DIRS4[(i + 1) % 4]` turns right and `DIRS4[(i + 3) % 4]` turns left. An index is handy as part of a search state or as an array index.

## `Grid`

A rectangular grid of bytes, stored row by row in one `Vec<u8>`.

| Item | Use |
| --- | --- |
| `Grid::parse(input)` | Parse a character grid. Panics if the rows differ in length |
| `Grid::new(w, h, b'.')` | A grid of the given size, filled with one byte |
| `g.width`, `g.height`, `g.cells` | Size, and the raw bytes |
| `g[p]`, `g[p] = b'#'` | Read or write a cell. Panics outside the grid |
| `g.get(p) -> Option<u8>` | Read a cell, or `None` outside the grid |
| `g.set(p, b'#')` | The same as `g[p] = b'#'` |
| `g.contains(p)` | Whether `p` is inside the grid |
| `g.find(b'S')` | The first position holding that byte |
| `g.positions()` | Every position, row by row |
| `g.neighbours(p, &DIRS4)` | The neighbours of `p` that are inside the grid (or `&DIRS8`) |
| `g.rows()` | Each row as `&[u8]` |
| `println!("{g}")` | Print the grid, for debugging |

- Cells are bytes, so compare with byte literals: `g[p] == b'#'`.
- For a digit grid, `(g[p] - b'0') as u32` gives the number.
- `Grid` is `Clone + Eq + Hash`, so a whole grid can be a key in a map, for example in [cycle detection](algorithms/cycles.md).
- For extra data per cell, such as a distance, use a `Vec` of size `width * height` indexed by `(r * width + c) as usize`. It is much faster than a map.

## Searches

| Item | Use |
| --- | --- |
| `bfs(start, next)` | The fewest steps from `start` to every reachable state. `next(&s)` returns the neighbours |
| `dijkstra(start, next, goal)` | `Some((cost, state))` for the cheapest state where `goal` holds. `next(&s)` returns `(neighbour, step_cost)` pairs |
| `dijkstra_all(start, next)` | The cheapest cost to every reachable state |

A state is any type that is `Clone + Eq + Hash`. Dijkstra states must also be `Ord`, which tuples of numbers are. `next` can return any iterator or collection; when the borrow checker objects to returning an iterator, add `.collect_vec()`.

See [Graph search](algorithms/graph-search.md) for how they work and how to design the state.

## Maths

| Item | Use |
| --- | --- |
| `gcd(a, b)` | Greatest common divisor, for `i64`. Never negative |
| `lcm(a, b)` | Least common multiple, for `i64` |

For a list, fold: `periods.iter().fold(1, |acc, &p| lcm(acc, p))`.

## Re-exports

| Item | Use |
| --- | --- |
| `Itertools` | Extra iterator methods; see [Parsing and iterators](parsing.md) |
| `FxHashMap`, `FxHashSet` | Faster hash maps for integer and tuple keys. Make one with `FxHashMap::default()`, not `new()` |
| `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet` | The std maps and sets. The `BTree` ones keep keys sorted |
| `VecDeque` | A queue: `push_back`, `pop_front` |
| `BinaryHeap`, `Reverse` | A max-heap; wrap items in `Reverse` for a min-heap |
| `Ordering` | The result of `cmp`, for custom sorting |
