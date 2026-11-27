# Algorithms

Most puzzles are one of a few known problems in disguise. Spot which one, and the solution follows. This page maps what a puzzle says to the algorithm to try. The other pages explain each algorithm: the idea, why it works, how to write it, how fast it is, and where it goes wrong.

## Recognising the puzzle

| The puzzle says or needs | Try | Page |
| --- | --- | --- |
| Fewest steps through a maze or grid; every move costs the same | BFS | [Graph search](graph-search.md#breadth-first-search-bfs) |
| Cheapest path where moves cost different amounts (turning costs 1000) | Dijkstra | [Graph search](graph-search.md#dijkstra) |
| Shortest path, but the plain search is too slow and the goal is a known point | A* | [Graph search](graph-search.md#a) |
| Movement rules depend on where you came from, how far you have gone straight, keys held | Put that in the search state | [Graph search](graph-search.md#designing-the-state) |
| Every cell on any shortest path | Search from both ends | [Graph search](graph-search.md#every-cell-on-a-shortest-path) |
| Regions of the same letter, areas, perimeters, enclosed space | Flood fill | [Graph search](graph-search.md#flood-fill) |
| An order that respects "X before Y" rules; dependencies | Topological sort | [Graph search](graph-search.md#topological-sort) |
| How many ways / arrangements; the same sub-question repeats | Memoisation | [Dynamic programming](dynamic-programming.md#memoisation) |
| Best score over choices made step by step along a line | DP table | [Dynamic programming](dynamic-programming.md#dp-tables) |
| Things multiply or split each step, and the count explodes | Count per kind | [Dynamic programming](dynamic-programming.md#count-per-kind) |
| A small set of switches, keys or visited nodes inside the state | Bit sets | [Dynamic programming](dynamic-programming.md#bit-sets) |
| Run a simulation 1,000,000,000 times | Cycle detection | [Cycles](cycles.md#cycle-detection) |
| Several independent things loop; when do they all line up? | LCM | [Cycles](cycles.md#several-cycles-lining-up-lcm) |
| `x mod a = r`, `x mod b = s`, ...; find `x` | Chinese remainder theorem | [Cycles](cycles.md#chinese-remainder-theorem) |
| Ranges that overlap; counting covered numbers; huge ranges | Interval merging | [Geometry](geometry.md#intervals) |
| Area inside a loop; dig plans; points enclosed by a path | Shoelace and Pick | [Geometry](geometry.md#area-inside-a-loop) |
| Few distinct coordinates, but huge values | Coordinate compression | [Geometry](geometry.md#coordinate-compression) |
| Diamond shapes, "within distance d" | Manhattan rotation | [Geometry](geometry.md#manhattan-diamonds) |
| The smallest value for which a condition starts to hold | Binary search on the answer | [Search and structure](search-and-structure.md#binary-search-on-the-answer) |
| Fill slots under constraints; try choices and undo | Backtracking | [Search and structure](search-and-structure.md#backtracking) |
| Groups that merge as connections are added | Union-find | [Search and structure](search-and-structure.md#union-find) |

## Judging speed before coding

A Rust solution does roughly 100 to 1000 million simple steps per second. Estimate the work before writing a slow plan:

| Work | Fine up to roughly |
| --- | --- |
| `n` (one pass) | Any input size |
| `n log n` (sort, heap) | Any input size |
| `n²` (every pair) | `n` = 10,000 to 30,000 |
| `n³` | `n` = 500 to 1,000 |
| `2ⁿ` (every subset) | `n` = 20 to 25 |
| `n!` (every order) | `n` = 10 to 11 |

When the estimate is too big, the puzzle wants a smarter idea: a cycle, counting per kind, maths, or a property of the real input.
