# Graph search

A lot of puzzles are graphs even when they do not say so. A **graph** is a set of **states** (nodes) and **moves** between them (edges). In a maze, a state is a position and a move is one step. In harder puzzles a state holds more, such as a position and a direction. Once you can say "from this state, I can reach these states", the searches below do the rest.

The `aoc` helpers take that description as a function:

```rust
bfs(start, |state| neighbours_of(state))
dijkstra(start, |state| neighbours_with_costs(state), |state| is_goal(state))
```

## Breadth-first search (BFS)

**Use when:** you want the fewest moves, and every move costs the same.

**Idea:** explore in rings. First the start, then every state 1 step away, then every state 2 steps away, and so on. A queue (first in, first out) gives this order: states found earlier are expanded earlier. The first time a state is found, it is found by a shortest route, because every state at a smaller distance was found before it.

**Steps:**

1. Put the start in a queue with distance 0, and mark it seen.
2. Take the front state from the queue.
3. For each neighbour not yet seen: mark it seen with distance + 1, and add it to the back of the queue.
4. Repeat from step 2 until the queue is empty.

**Cost:** each state and each move is handled once, so the time grows with the size of the graph.

**With the helper:** `bfs` returns the distance to every reachable state.

```rust
let g = Grid::parse(input);
let (start, end) = (g.find(b'S').unwrap(), g.find(b'E').unwrap());
let dist = bfs(start, |&p| {
    g.neighbours(p, &DIRS4).filter(|&n| g[n] != b'#').collect_vec()
});
let steps = dist[&end]; // panics if `end` cannot be reached; use dist.get(&end) to check
```

The returned map is useful on its own: `dist.len()` is the number of reachable states, and `dist.values().filter(|&&d| d <= 64).count()` counts states within 64 steps.

**Written by hand,** when you want to stop early or carry extra data:

```rust
let mut seen = FxHashSet::default();
let mut queue = VecDeque::from([(start, 0)]);
seen.insert(start);
while let Some((p, d)) = queue.pop_front() {
    if p == end {
        return d;
    }
    for n in g.neighbours(p, &DIRS4) {
        if g[n] != b'#' && seen.insert(n) {
            queue.push_back((n, d + 1));
        }
    }
}
```

`seen.insert(n)` returns `false` when `n` was already there, so it checks and marks in one call.

**Pitfalls:**

- Mark a state seen when you **add** it to the queue, not when you take it out. Otherwise one state can be queued many times.
- BFS gives wrong answers when moves cost different amounts. Use Dijkstra.

## Dijkstra

**Use when:** you want the cheapest path, moves cost different amounts, and no cost is negative.

**Idea:** like BFS, but expand states in order of total cost so far, not in order of steps. A priority queue (a heap) always hands back the cheapest state found so far. When a state comes out of the heap, no cheaper route to it can exist: every other route would pass through a state still in the heap, which already costs at least as much, and costs never go down.

**Steps:**

1. Put the start in the heap with cost 0.
2. Take the cheapest state from the heap. If a cheaper cost for it is already known, skip it; this entry is stale.
3. If it is the goal, its cost is the answer.
4. For each move, compute the new cost. If it beats the best known cost of that neighbour, record it and push the neighbour onto the heap.
5. Repeat from step 2.

A state can be pushed several times, once for each improvement. Step 2 throws the old entries away. That is simpler than updating an entry inside the heap, and just as fast in practice.

**Cost:** about `E log V` for `E` moves and `V` states, because of the heap.

**With the helper:** `next` returns `(neighbour, step_cost)` pairs. The result is the goal's cost and the goal state itself.

```rust
// A maze where stepping costs 1 and turning 90 degrees costs 1000.
// The state is (position, direction), because the cost of a move depends on the direction.
let start = (g.find(b'S').unwrap(), (0, 1)); // facing right
let (cost, _) = dijkstra(
    start,
    |&(p, d)| {
        let mut moves = vec![((p, turn_left(d)), 1000), ((p, turn_right(d)), 1000)];
        if g[add(p, d)] != b'#' {
            moves.push(((add(p, d), d), 1));
        }
        moves
    },
    |&(p, _)| g[p] == b'E',
)
.unwrap();
```

**Pitfalls:**

- Negative costs break it. Advent of Code almost never has them.
- With unit costs, Dijkstra still works, but BFS is simpler and faster.
- If the goal is "any state at position E", say so in `goal`. Do not look up one exact state, since the direction at the end is unknown.

## A*

**Use when:** Dijkstra is too slow and you are going to one known goal.

**Idea:** Dijkstra expands in every direction equally. A* adds a guess of the remaining cost, the **heuristic** `h(state)`, and expands in order of `cost so far + h`. States heading towards the goal come out first. For a grid with unit steps, `h` is the Manhattan distance to the goal.

The heuristic must never overestimate the real remaining cost, or A* can return a path that is too expensive. Manhattan distance is safe on a grid without diagonal moves, because no path can be shorter than it.

**With the helper:** A* is Dijkstra with adjusted step costs. Give each move the cost `step + h(to) - h(from)`. The total along any path then becomes `real cost + h(goal) - h(start)`, so the cheapest path is unchanged and `real cost = result + h(start)` when `h(goal) = 0`. The adjusted cost is never negative when `h` drops by at most the step cost on every move, which Manhattan distance does on a grid.

```rust
let h = |p: Pos| manhattan(p, end) as u64;
let (adjusted, _) = dijkstra(
    start,
    |&p| {
        g.neighbours(p, &DIRS4)
            .filter(|&n| g[n] != b'#')
            .map(|n| (n, 1 + h(n) - h(p))) // never negative: h drops by at most 1 per step
            .collect_vec()
    },
    |&p| p == end,
)
.unwrap();
let steps = adjusted + h(start);
```

In practice A* rarely matters for Advent of Code. A better state design usually helps more.

## Designing the state

The state must hold everything that affects **which moves are allowed next** and **what they cost**. Nothing more, because every extra field multiplies the number of states.

| Puzzle rule | State |
| --- | --- |
| Plain maze | `Pos` |
| Turning costs extra | `(Pos, direction)` |
| At most 3 steps in a straight line | `(Pos, direction, steps_in_a_row)` |
| Collect keys, doors need keys | `(Pos, keys as a u64 bit set)` |
| Can pass through one wall once (a cheat) | `(Pos, cheat_used: bool)` |
| Several robots moving | A tuple or array of all their positions |

Two states that behave the same in the future should be equal. For example, if the remaining moves do not depend on the path taken, do not store the path in the state.

## Every cell on a shortest path

Run `dijkstra_all` (or `bfs`) twice: once from the start, and once from the goal with the moves reversed. A state `s` is on some shortest path exactly when

```
cost_from_start[s] + cost_to_goal[s] == best_total
```

because a shortest path through `s` is a shortest path to `s` followed by a shortest path from `s`.

For "reversed moves" with direction states: the backward search from `(p, d)` steps to `(p - d, d)` and turns in place. Start it from every `(goal, d)` with cost 0, for example by searching from a special start state, or by running it once per direction and taking the minimum.

The same two-search trick answers "how much does a shortcut save": a cheat from `a` to `b` gives a path of `from_start[a] + cheat_length + to_goal[b]`.

## Flood fill

**Use when:** you need connected regions, such as plots of the same letter, or the open space enclosed by a wall.

**Idea:** a BFS (or DFS) that only moves between cells of the same kind. Start it from every cell not yet visited, and each run finds one whole region.

```rust
// Regions of equal letters, with area and perimeter.
let mut seen = vec![false; g.cells.len()];
let idx = |(r, c): Pos| (r * g.width + c) as usize;
let mut total = 0;
for p in g.positions() {
    if seen[idx(p)] {
        continue;
    }
    let region = bfs(p, |&q| {
        g.neighbours(q, &DIRS4).filter(|&n| g[n] == g[q]).collect_vec()
    });
    let area = region.len();
    // Each cell side that does not touch the same letter is part of the perimeter.
    let perimeter: usize = region
        .keys()
        .map(|&q| DIRS4.iter().filter(|&&d| g.get(add(q, d)) != Some(g[q])).count())
        .sum();
    for &q in region.keys() {
        seen[idx(q)] = true;
    }
    total += area * perimeter;
}
```

**Counting sides instead of perimeter:** count corners instead, because a polygon has as many sides as corners. A cell has an outer corner where two neighbouring sides (such as up and right) are both outside the region. It has an inner corner where both those sides are inside but the diagonal between them is outside.

**Enclosed space:** to find cells inside a loop, flood fill from outside the loop (pad the grid with a border so everything outside is connected). Whatever the fill did not reach and is not the loop itself is inside. For loops drawn by a path, the shoelace formula in [Geometry](geometry.md#area-inside-a-loop) is often easier.

## Topological sort

**Use when:** rules say "X must come before Y", and you need an order that obeys all of them, or need to check whether an order does.

**Idea (Kahn's algorithm):** count, for each item, how many items must come before it. Items with count 0 can go first. Output one, then lower the counts of the items that wait for it; any that reach 0 are ready. If items are left over at the end, the rules contain a loop and no order exists.

```rust
// rules: Vec<(u32, u32)> meaning "a before b". items: the things to order.
fn topo_sort(items: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut waiting_on: FxHashMap<u32, usize> = items.iter().map(|&i| (i, 0)).collect();
    let mut after: FxHashMap<u32, Vec<u32>> = FxHashMap::default();
    for &(a, b) in rules {
        if waiting_on.contains_key(&a) && waiting_on.contains_key(&b) {
            *waiting_on.get_mut(&b).unwrap() += 1;
            after.entry(a).or_default().push(b);
        }
    }
    let mut ready: VecDeque<u32> =
        items.iter().copied().filter(|i| waiting_on[i] == 0).collect();
    let mut order = Vec::new();
    while let Some(i) = ready.pop_front() {
        order.push(i);
        for &j in after.get(&i).into_iter().flatten() {
            let count = waiting_on.get_mut(&j).unwrap();
            *count -= 1;
            if *count == 0 {
                ready.push_back(j);
            }
        }
    }
    order // shorter than `items` if the rules have a loop
}
```

Only the rules between the items at hand count. A full rule set can have loops while every subset used in the puzzle has none, so filter the rules first, as above.

**Shortcut:** when the rules cover every pair, sorting with a comparator works too: `pages.sort_by(|a, b| if rules.contains(&(*a, *b)) { Ordering::Less } else { Ordering::Greater })`.

**Counting paths in a DAG** (a graph with no loops): the number of paths from `a` to `z` is the sum of the path counts from each neighbour of `a`. Memoise it as in [Dynamic programming](dynamic-programming.md#memoisation).

## Further reading

- [Red Blob Games: Introduction to A*](https://www.redblobgames.com/pathfinding/a-star/introduction.html) shows BFS, Dijkstra and A* side by side, with animations.
- [Red Blob Games: Grids and graphs](https://www.redblobgames.com/pathfinding/grids/graphs.html).
