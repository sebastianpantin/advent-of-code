//! Shared helpers for the daily solutions in `YEAR/src/bin/dayNN.rs`.

pub mod search;
pub mod tools;

// Everyday imports, so a day file needs only `use aoc::*;`.
pub use itertools::Itertools;
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use search::{bfs, dijkstra, dijkstra_all};
pub use std::cmp::{Ordering, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::path::PathBuf;
use std::time::Instant;

/// Read the puzzle input for the calling day. The year and day come from the crate and
/// binary names (`aoc2026`, `day01`), so a day file needs no constants and can be copied.
#[macro_export]
macro_rules! read_input {
    () => {
        $crate::read_input_for(env!("CARGO_PKG_NAME"), env!("CARGO_BIN_NAME"))
    };
}

/// Read `inputs/YEAR/dayNN.txt` for the package `aocYEAR` and binary `dayNN`.
pub fn read_input_for(package: &str, bin: &str) -> String {
    let parse = |name: &str, prefix: &str| -> u32 {
        name.strip_prefix(prefix)
            .and_then(|n| n.parse().ok())
            .unwrap_or_else(|| panic!("`{name}` is not named like `{prefix}N`"))
    };
    let (year, day) = (parse(package, "aoc"), parse(bin, "day"));
    let path = input_path(year, day);
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "cannot read {}: {e} (run `cargo new-day {year} {day}`)",
            path.display()
        )
    })
}

/// The workspace root (set by `AOC_ROOT` in `.cargo/config.toml`), so paths work
/// whichever directory cargo is run from.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("AOC_ROOT"))
}

/// The crate holding a year's solutions.
pub fn package_name(year: u32) -> String {
    format!("aoc{year}")
}

/// The binary holding a day's solution.
pub fn bin_name(day: u32) -> String {
    format!("day{day:02}")
}

/// Where a day's puzzle input lives.
pub fn input_path(year: u32, day: u32) -> PathBuf {
    workspace_root().join(format!("inputs/{year}/{}.txt", bin_name(day)))
}

/// Run one part, then print its answer and how long it took.
pub fn timed<T: Display>(label: &str, f: impl FnOnce() -> T) {
    let start = Instant::now();
    let answer = f();
    println!("{label}: {answer}  ({:.2?})", start.elapsed());
}

/// Every integer in the text, signed, in order. Handy for lines like `x=-3, y=14`.
pub fn ints(s: &str) -> Vec<i64> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let negative = bytes[i] == b'-' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        if negative || bytes[i].is_ascii_digit() {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            out.push(s[start..i].parse().unwrap());
        } else {
            i += 1;
        }
    }
    out
}

/// A position as (row, column). Signed, so stepping off the edge is safe to compute.
pub type Pos = (i32, i32);

/// Up, right, down, left.
pub const DIRS4: [Pos; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// The four straight directions plus the four diagonals, clockwise from up.
pub const DIRS8: [Pos; 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[inline]
pub fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

/// The direction 90 degrees clockwise, e.g. up to right.
#[inline]
pub fn turn_right((r, c): Pos) -> Pos {
    (c, -r)
}

/// The direction 90 degrees anticlockwise, e.g. up to left.
#[inline]
pub fn turn_left((r, c): Pos) -> Pos {
    (-c, r)
}

/// The number of straight steps between two positions.
#[inline]
pub fn manhattan(a: Pos, b: Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// The greatest common divisor, never negative.
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

/// The least common multiple, e.g. when several cycles line up.
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b) * b).abs()
    }
}

/// A rectangular character grid, stored row by row.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub cells: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Grid {
    /// A grid of the given size with every cell set to `fill`.
    pub fn new(width: i32, height: i32, fill: u8) -> Self {
        Grid {
            cells: vec![fill; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn parse(s: &str) -> Self {
        let lines: Vec<&[u8]> = s.lines().map(str::as_bytes).collect();
        let width = lines.first().map_or(0, |l| l.len());
        assert!(
            lines.iter().all(|l| l.len() == width),
            "grid rows differ in length"
        );
        Grid {
            cells: lines.concat(),
            width: width as i32,
            height: lines.len() as i32,
        }
    }

    #[inline]
    pub fn contains(&self, (r, c): Pos) -> bool {
        (0..self.height).contains(&r) && (0..self.width).contains(&c)
    }

    #[inline]
    pub fn get(&self, p: Pos) -> Option<u8> {
        self.contains(p)
            .then(|| self.cells[(p.0 * self.width + p.1) as usize])
    }

    #[inline]
    pub fn set(&mut self, p: Pos, value: u8) {
        self[p] = value;
    }

    /// Every position, row by row.
    pub fn positions(&self) -> impl Iterator<Item = Pos> + use<> {
        let (h, w) = (self.height, self.width);
        (0..h).flat_map(move |r| (0..w).map(move |c| (r, c)))
    }

    /// The first position holding `value`, e.g. the `S` start marker.
    pub fn find(&self, value: u8) -> Option<Pos> {
        let w = self.width as usize;
        let i = self.cells.iter().position(|&b| b == value)?;
        Some(((i / w) as i32, (i % w) as i32))
    }

    /// The in-bounds neighbours of `p` in the given directions (`DIRS4` or `DIRS8`).
    #[inline]
    pub fn neighbours<'a>(&'a self, p: Pos, dirs: &'a [Pos]) -> impl Iterator<Item = Pos> + 'a {
        dirs.iter()
            .map(move |&d| add(p, d))
            .filter(|&n| self.contains(n))
    }

    /// Each row as a byte slice, top to bottom.
    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.cells.chunks(self.width.max(1) as usize)
    }

    #[inline]
    fn index(&self, (r, c): Pos) -> usize {
        assert!(self.contains((r, c)), "({r}, {c}) is outside the grid");
        (r * self.width + c) as usize
    }
}

/// `grid[p]` reads a cell and panics outside the grid; use `get` to check instead.
impl Index<Pos> for Grid {
    type Output = u8;

    #[inline]
    fn index(&self, p: Pos) -> &u8 {
        &self.cells[Grid::index(self, p)]
    }
}

impl IndexMut<Pos> for Grid {
    #[inline]
    fn index_mut(&mut self, p: Pos) -> &mut u8 {
        let i = Grid::index(self, p);
        &mut self.cells[i]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.rows() {
            writeln!(f, "{}", String::from_utf8_lossy(row))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ints_finds_signed_numbers_and_ignores_lone_dashes() {
        assert_eq!(ints("p=-3,14 v=2--7 a - b 08"), vec![-3, 14, 2, -7, 8]);
    }

    #[test]
    fn grid_parses_finds_and_bounds_neighbours() {
        let g = Grid::parse("#S.\n..#\n");
        assert_eq!((g.width, g.height), (3, 2));
        assert_eq!(g.find(b'S'), Some((0, 1)));
        assert_eq!(g.get((1, 2)), Some(b'#'));
        assert_eq!(g.get((2, 0)), None);
        let n: Vec<Pos> = g.neighbours((0, 0), &DIRS4).collect();
        assert_eq!(n, vec![(0, 1), (1, 0)]);
        assert_eq!(g.to_string(), "#S.\n..#\n");
    }

    #[test]
    fn grid_index_reads_and_writes_cells() {
        let mut g = Grid::new(2, 2, b'.');
        g[(1, 0)] = b'#';
        assert_eq!(g[(1, 0)], b'#');
        assert_eq!(g.to_string(), "..\n#.\n");
    }

    #[test]
    fn turns_cycle_through_dirs4() {
        for i in 0..4 {
            assert_eq!(turn_right(DIRS4[i]), DIRS4[(i + 1) % 4]);
            assert_eq!(turn_left(DIRS4[i]), DIRS4[(i + 3) % 4]);
        }
    }

    #[test]
    fn gcd_and_lcm() {
        assert_eq!(gcd(12, -18), 6);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(0, 6), 0);
    }
}
