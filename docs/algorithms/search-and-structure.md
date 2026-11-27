# Search and structure

## Binary search on the answer

**Use when:** the question is "the smallest (or largest) value for which something holds", and the condition flips only once: false for every value below some point, true for every value from that point on. Examples: the first byte that blocks the path, the smallest number of steps after which a pattern appears, the largest input a machine accepts.

**Idea:** check the middle of the range. If the condition holds there, the answer is in the lower half; otherwise it is in the upper half. Each check halves the range, so a range of a billion values needs about 30 checks. Each check can be slow (a whole BFS) and the search is still fast.

```rust
/// The smallest value in lo..hi where `holds` is true, or hi if there is none.
/// `holds` must be false for every value below the answer and true from it on.
fn first_true(mut lo: i64, mut hi: i64, holds: impl Fn(i64) -> bool) -> i64 {
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if holds(mid) {
            hi = mid; // mid might be the answer; keep it
        } else {
            lo = mid + 1; // mid is too small
        }
    }
    lo
}
```

**Example:** the first number of fallen bytes that blocks every path:

```rust
let blocked = first_true(0, bytes.len() as i64, |n| {
    let walls: FxHashSet<Pos> = bytes[..n as usize].iter().copied().collect();
    !bfs((0, 0), |&p| {
        DIRS4.iter()
            .map(move |&d| add(p, d))
            .filter(|&q| (0..=70).contains(&q.0) && (0..=70).contains(&q.1) && !walls.contains(&q))
            .collect_vec()
    })
    .contains_key(&(70, 70))
});
// `blocked` bytes block the path, so the culprit is bytes[blocked - 1].
```

On a sorted slice, std already has it: `slice.partition_point(|x| !holds(x))`.

**Pitfall:** the condition must flip only once. If it can go true, false, true, binary search gives a wrong answer without any error.

## Backtracking

**Use when:** you must choose values for several slots under constraints (fill in operators, place pieces, assign wires), and you want one valid choice, or all of them.

**Idea:** a depth-first search over choices. Pick a value for the first slot, then recurse on the rest. If a choice leads to a dead end, undo it and try the next. The key to speed is **pruning**: stop going deeper as soon as the partial choice cannot work.

**Example:** can `+` and `*` between the numbers, evaluated left to right, give the target?

```rust
fn can_make(target: u64, acc: u64, rest: &[u64]) -> bool {
    if acc > target {
        return false; // prune: + and * only make it bigger
    }
    match rest {
        [] => acc == target,
        [n, tail @ ..] => can_make(target, acc + n, tail) || can_make(target, acc * n, tail),
    }
}
// can_make(target, nums[0], &nums[1..])
```

`||` stops at the first success, so the search ends as soon as one answer is found.

**Working backwards** prunes even more here: from the last number, `target - n` must be reachable, and `target / n` only when `n` divides `target`. Many branches end at once.

**Pitfalls:**

- Without pruning, backtracking is `choices ^ slots`. Check that number first; see [Judging speed](README.md#judging-speed-before-coding).
- If you change shared state (a grid, a set) while going deeper, undo the change before trying the next choice.

## Union-find

**Use when:** items join into groups as connections are added, and you need to know which items are in the same group, how many groups there are, or how big they are. Example: connect the closest pairs of boxes one by one, and stop when everything is one circuit.

**Idea:** each group is a tree, and the root names the group. `find(x)` follows parents up to the root. `union(a, b)` makes one root the parent of the other. Two tricks keep the trees flat, so both operations take almost constant time:

- **Path compression:** after `find`, point every node on the way straight at the root.
- **Union by size:** hang the smaller tree under the bigger one.

```rust
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), size: vec![1; n], groups: n }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Join the groups of a and b. Returns false if they were already one group.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.groups -= 1;
        true
    }
}
```

- Group sizes: the `size` of each root, that is, each `i` where `find(i) == i`.
- **Minimum spanning tree (Kruskal):** sort all pairs by distance, and `union` them in that order. The pairs where `union` returns `true` form the cheapest set of links that connects everything. The last such pair is the one that makes it a single group.

For a one-off "which items are connected" question with all links known up front, a [flood fill](graph-search.md#flood-fill) over the links works too.

## Further reading

- [CP-Algorithms: Binary search](https://cp-algorithms.com/num_methods/binary_search.html).
- [CP-Algorithms: Disjoint set union](https://cp-algorithms.com/data_structures/disjoint_set_union.html).
