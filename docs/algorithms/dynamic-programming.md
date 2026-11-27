# Dynamic programming

**Dynamic programming** (DP) solves a problem by splitting it into smaller problems of the same shape, solving each smaller problem only once, and reusing the answers. It applies when:

1. the answer can be built from answers to smaller versions of the question, and
2. the same smaller questions come up again and again.

Without reuse, the work can grow exponentially. With reuse, it is the number of distinct sub-questions times the work per sub-question.

Typical puzzle words: "how many different ways", "how many arrangements", "the best total", "the number of possible ...", and answers far too big to count one by one.

## Memoisation

**Idea:** write the plain recursive solution, then store each result in a map keyed by the arguments. Before computing, look in the map. This is "top-down" DP and is usually the easiest to get right.

**Steps:**

1. Write a recursive function that answers the question for a given sub-problem.
2. Decide its **key**: the smallest set of values that fully decides the answer.
3. Look up the key first; return the stored result if there is one.
4. Otherwise compute, store, and return.

**Example:** how many ways can a string be built from a list of pieces? (`"abab"` from `a`, `b`, `ab` can be built 4 ways.)

```rust
fn ways<'a>(s: &'a str, pieces: &[&str], memo: &mut FxHashMap<&'a str, u64>) -> u64 {
    if s.is_empty() {
        return 1; // one way to build nothing: use no pieces
    }
    if let Some(&n) = memo.get(s) {
        return n;
    }
    let n = pieces
        .iter()
        .filter_map(|p| s.strip_prefix(p))
        .map(|rest| ways(rest, pieces, memo))
        .sum();
    memo.insert(s, n);
    n
}
```

Here the key is the remaining string. Since it is always a suffix of the same string, its length would work as a cheaper key.

**Choosing the key:** include only what changes the answer. In a spring-row puzzle, the answer depends on "position in the row" and "how many groups are left", not on the choices already made. So the key is `(position, groups_done)`, and there are only `row_length * group_count` distinct calls.

**Pitfalls:**

- If the function depends on something not in the key (a global, a field), the stored results are wrong.
- Rust closures cannot call themselves, so write a `fn` and pass the memo and any shared data as arguments.
- Very deep recursion can overflow the stack. Usual puzzle depths are fine; if not, use a table instead.

## DP tables

**Idea:** instead of recursing from the big question down, fill a table from the smallest questions up. "Bottom-up" DP. It needs the order of sub-problems to be clear, but has no recursion and no hashing, so it is faster.

**Example:** count paths from the top row to the bottom of a grid, where each step goes down, or down and one column left or right. Each cell's count is the sum of the counts of the cells that can reach it.

```rust
let g = Grid::parse(input);
let w = g.width as usize;
let mut counts = vec![0u64; w];
counts[g.find(b'S').unwrap().1 as usize] = 1;
for row in g.rows().skip(1) {
    let mut next = vec![0u64; w];
    for c in 0..w {
        if row[c] == b'#' {
            continue;
        }
        for from in c.saturating_sub(1)..=(c + 1).min(w - 1) {
            next[c] += counts[from];
        }
    }
    counts = next;
}
let total: u64 = counts.iter().sum();
```

Only the previous row is needed for the next, so the table is one row long. Many DP tables shrink this way.

## Count per kind

**Use when:** things multiply or split every step (fish, stones, particles), but many of them are identical.

**Idea:** two things with the same value behave the same forever. So keep **how many** of each value exist, not a list of every item. A list of billions of numbers becomes a map of a few thousand counts.

```rust
let mut counts: FxHashMap<u64, u64> = FxHashMap::default();
for n in ints(input) {
    *counts.entry(n as u64).or_default() += 1;
}
for _ in 0..75 {
    let mut next = FxHashMap::default();
    for (&n, &c) in &counts {
        for m in step(n) {
            // every one of the `c` copies of `n` becomes an `m`
            *next.entry(m).or_default() += c;
        }
    }
    counts = next;
}
let total: u64 = counts.values().sum();
```

When the values are small (like the ages 0 to 8), use an array instead of a map: `let mut counts = [0u64; 9];`.

The same idea shows up as **"the order does not matter"**: if the puzzle only asks for totals, drop whatever detail does not change the totals.

## Bit sets

**Use when:** a state includes a small set, such as keys collected, valves opened or nodes visited, with at most 64 possible members.

**Idea:** give each member a number from 0 to 63 and store the set as the bits of a `u64`. Bit `i` is 1 when member `i` is in the set. The set is then a plain number: cheap to copy, hash and compare, so it can go in a search state or a memo key.

| Operation | Code |
| --- | --- |
| Empty set | `0u64` |
| Add `i` | `set \| 1 << i` |
| Remove `i` | `set & !(1 << i)` |
| Contains `i`? | `set & 1 << i != 0` |
| Size | `set.count_ones()` |
| Union, intersection | `a \| b`, `a & b` |
| Is `a` a subset of `b`? | `a & !b == 0` |
| Every subset of `n` members | `for set in 0..1u64 << n` |
| Every subset of `mask` | `let mut s = mask; loop { ...; if s == 0 { break } s = (s - 1) & mask; }` |

**Example: the travelling-salesman style search.** The best route through a set of nodes, ending at node `last`, depends only on `(set visited, last)`. That is `2ⁿ * n` states instead of `n!` orders. With `n = 15` that is about half a million states, against more than a trillion orders.

Map names to numbers first; see [Parsing](../parsing.md#names-to-numbers).

## Further reading

- [Wikipedia: Dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
- [CP-Algorithms: Introduction to DP](https://cp-algorithms.com/dynamic_programming/intro-to-dp.html).
