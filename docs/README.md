# Docs

Reference notes for solving Advent of Code in this workspace.

- [Workflow](workflow.md): setup, the daily commands, the day file, and general tips.
- [Helpers](helpers.md): everything `use aoc::*;` gives you.
- [Parsing and iterators](parsing.md): turning input into data, and the iterator methods worth knowing.
- [Algorithms](algorithms/README.md): how to recognise a puzzle type, then how each algorithm works.
  - [Graph search](algorithms/graph-search.md): BFS, Dijkstra, A*, flood fill, topological sort.
  - [Dynamic programming](algorithms/dynamic-programming.md): memoisation, tables, counting per kind, bit sets.
  - [Cycles and number theory](algorithms/cycles.md): cycle detection, LCM, the Chinese remainder theorem.
  - [Geometry](algorithms/geometry.md): intervals, polygon area, coordinate compression, Manhattan tricks.
  - [Search and structure](algorithms/search-and-structure.md): binary search on the answer, backtracking, union-find.

Run `cargo doc -p aoc --open` for the generated API docs.
