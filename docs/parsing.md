# Parsing and iterators

## Parsing patterns

Puzzle inputs end with a newline. `lines()` handles that; `split('\n')` gives an empty last item. Use `input.trim()` when splitting by hand.

```rust
// One number per line.
let nums: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();

// Two sections separated by a blank line.
let (rules, updates) = input.split_once("\n\n").unwrap();

// Any number of blank-line separated blocks.
let blocks: Vec<&str> = input.split("\n\n").collect();

// Lines with numbers in them, whatever the punctuation.
let rows: Vec<Vec<i64>> = input.lines().map(ints).collect();

// A fixed shape per line, e.g. "move 3 from 1 to 2".
let [n, from, to] = ints(line)[..] else { panic!("bad line: {line}") };

// Key and value.
let (name, rest) = line.split_once(": ").unwrap();

// Words.
let words: Vec<&str> = line.split_whitespace().collect();

// Digits, e.g. "12345".
let digits: Vec<u32> = line.bytes().map(|b| (b - b'0') as u32).collect();

// Ranges like "2-7" (not `ints`, which reads "-7" as negative).
let (lo, hi) = line.split_once('-').unwrap();
let (lo, hi): (u64, u64) = (lo.parse().unwrap(), hi.parse().unwrap());

// Column-major data: transpose the rows.
let cols: Vec<Vec<i64>> = (0..rows[0].len())
    .map(|c| rows.iter().map(|r| r[c]).collect())
    .collect();
```

### Names to numbers

Graph puzzles often name nodes with strings like `AA` or `svr`. Strings work as map keys, but small integer ids are faster and fit in `Vec`s and bit sets:

```rust
let mut ids: FxHashMap<&str, usize> = FxHashMap::default();
let mut id = |name| {
    let next = ids.len();
    *ids.entry(name).or_insert(next)
};
let a = id("AA"); // 0
let b = id("BB"); // 1
```

## Iterator methods worth knowing

### Std

| Method | Use |
| --- | --- |
| `slice.windows(2)` | Overlapping runs: `[a, b, c]` gives `[a, b]`, `[b, c]` |
| `slice.chunks(3)` | Non-overlapping runs |
| `a.iter().zip(b)` | Walk two lists side by side |
| `.enumerate()` | Items with their index |
| `.rev()`, `.step_by(n)`, `.skip(n)`, `.take(n)` | Order and spacing |
| `.filter_map(f)` | Filter and map at once; keep the `Some` results |
| `.flat_map(f)` | Map each item to many, then flatten |
| `.fold(init, f)`, `.scan(init, f)` | Running totals; `scan` yields each step |
| `.any(p)`, `.all(p)`, `.position(p)`, `.find(p)` | Searching |
| `.min_by_key(f)`, `.max_by_key(f)` | The best item by some measure |
| `.sum()`, `.product()`, `.count()` | Totals |
| `slice.sort_unstable()`, `.sort_by_key(f)` | Sorting in place |
| `slice.binary_search(&x)`, `.partition_point(p)` | Searching a sorted slice |
| `slice.rotate_left(n)`, `.swap(i, j)` | Rearranging |
| `iter::successors(first, f)` | A sequence where each item comes from the last |

### `Itertools`

| Method | Use |
| --- | --- |
| `.tuple_windows::<(_, _)>()` | Overlapping pairs, as tuples |
| `.circular_tuple_windows::<(_, _)>()` | The same, plus the pair from last back to first |
| `.tuples::<(_, _)>()` | Non-overlapping pairs |
| `.combinations(k)` | Every set of `k` items |
| `.tuple_combinations::<(_, _)>()` | Every unordered pair, as tuples |
| `.permutations(k)` | Every ordering of `k` items |
| `.cartesian_product(other)` | Every `(a, b)` pair from two iterators |
| `.counts()` | A `HashMap` from item to how many times it appears |
| `.sorted()`, `.sorted_by_key(f)` | Sort without a `mut` vector |
| `.dedup()`, `.unique()` | Drop repeats next to each other, or all repeats |
| `.minmax()` | The smallest and the largest in one pass |
| `.positions(p)` | The indexes where `p` holds |
| `.chunk_by(key)` | Group runs of items that share a key |
| `.collect_vec()` | Short for `.collect::<Vec<_>>()` |
| `.join(", ")` | Join items into a string |

Number of results, to judge whether brute force is fast enough: `n` items give `n * (n - 1) / 2` pairs, `n! / (k! * (n - k)!)` combinations of `k`, and `n!` permutations. `10! = 3_628_800` is fine; `15!` is not.
