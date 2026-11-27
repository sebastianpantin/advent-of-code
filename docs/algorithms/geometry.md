# Geometry

## Intervals

**Use when:** the input is ranges like `3-5` or `10-14`, possibly huge and overlapping, and the question is how many numbers they cover, or whether a number is in any of them.

**Idea:** sort the ranges by start. Then overlapping ranges are next to each other, and one pass merges them: if a range starts at or before the end of the last merged range, extend that one; otherwise start a new one.

```rust
// Inclusive ranges (start, end).
fn merge(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_unstable();
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (s, e) in ranges {
        match merged.last_mut() {
            // `+ 1` also joins ranges that touch, such as 1-3 and 4-6.
            Some(last) if s <= last.1 + 1 => last.1 = last.1.max(e),
            _ => merged.push((s, e)),
        }
    }
    merged
}
```

After merging:

- Numbers covered: `merged.iter().map(|(s, e)| e - s + 1).sum()`.
- Is `x` covered? The merged ranges are sorted and do not overlap, so binary search: `let i = merged.partition_point(|&(s, _)| s <= x);` then `x` is covered when `i > 0 && x <= merged[i - 1].1`.

**Inclusive or exclusive:** decide once. Inclusive `(s, e)` has `e - s + 1` numbers; half-open `s..e` has `e - s`. Mixing them up is the most common interval bug.

**Splitting a range by a rule** (a range of seeds passed through a mapping): cut the range into the part before the rule's range, the overlap, and the part after. Only the overlap moves. With `a..b` the range and `c..d` the rule, the overlap is `max(a, c)..min(b, d)`, and it is empty when `max(a, c) >= min(b, d)`.

## Area inside a loop

**Use when:** a path of straight moves, such as a dig plan, draws a closed loop, and you need the area inside or the number of cells inside.

**Shoelace formula:** for a polygon with corners `(x0, y0), (x1, y1), ..., (xn-1, yn-1)` in order around the shape,

```
2 * area = | sum over i of (xi * yi+1 - xi+1 * yi) |
```

where the corner after the last is the first. Each term is twice the signed area of the triangle from the origin to one edge. Edges going one way around add area and edges going back subtract it, so what remains is the area inside.

**Pick's theorem** links area to grid points. For a polygon whose corners are on whole-number points,

```
area = inside + boundary / 2 - 1
```

where `inside` is the number of grid points strictly inside and `boundary` is the number on the edge.

**Counting cells in a dug loop:** the loop's cells are the boundary points, and the dug-out cells are inside + boundary. Rearranging Pick: `inside = area - boundary / 2 + 1`, so `cells = area + boundary / 2 + 1`.

```rust
// corners: Vec<(i64, i64)> in order around the loop.
let twice_area: i64 = corners
    .iter()
    .circular_tuple_windows()
    .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
    .sum();
let boundary: i64 = corners
    .iter()
    .circular_tuple_windows()
    .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
    .sum();
let cells = twice_area.abs() / 2 + boundary / 2 + 1;
```

Build the corners by walking the instructions: start at `(0, 0)`, and add `direction * distance` for each one. This works even when the distances are in the millions, since nothing is drawn.

**Inside a pipe loop on a grid** (scan-line): walk each row left to right and flip "inside" every time you cross a wall that goes up. Count the cells passed while inside. Crossing `|` flips; for corner pairs such as `L---7` or `F---J`, count only one of each pair, for example only the pieces that connect upward (`|`, `L`, `J`).

## Coordinate compression

**Use when:** there are few distinct coordinates but the values are huge, so a grid of the real size will not fit.

**Idea:** only the coordinates that appear matter. Sort the distinct values, and use each value's index in the sorted list as its new coordinate. Gaps between values shrink to one step, but the order is kept, so "is left of" and "overlaps" stay true. When you need real sizes back, look up the original values.

```rust
let mut xs: Vec<i64> = points.iter().map(|p| p.0).collect();
xs.sort_unstable();
xs.dedup();
let compress = |x: i64| xs.binary_search(&x).unwrap();
// The real width of compressed column i is xs[i + 1] - xs[i].
```

For areas, also add each range's end + 1 to the list, so the cells between two values become their own column.

## Manhattan diamonds

Points within Manhattan distance `d` of a centre form a diamond, not a square. Rotating by 45 degrees turns diamonds into squares:

```
u = x + y
v = x - y
```

Then `manhattan(a, b) = max(|ua - ub|, |va - vb|)`, and "within distance `d`" becomes "inside a square of size `2d`". Squares are easier to merge, intersect and sweep. Convert back with `x = (u + v) / 2`, `y = (u - v) / 2`; only points where `u` and `v` have the same parity are real grid points.

**Rows only:** for "which columns in row `y` are within `d` of a sensor at `(sx, sy)`", the answer is one interval: `sx - r ..= sx + r` with `r = d - |y - sy|`, empty when `r < 0`. Merge the intervals from all sensors; see [Intervals](#intervals).

## Hexagonal grids

Use **cube coordinates** `(q, r, s)` with `q + r + s = 0`. Each of the six moves changes two coordinates, one by +1 and one by -1. The distance between two cells is `(|dq| + |dr| + |ds|) / 2`. See [Red Blob Games: Hexagonal grids](https://www.redblobgames.com/grids/hexagons/).

## Further reading

- [Wikipedia: Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula).
- [Wikipedia: Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem).
