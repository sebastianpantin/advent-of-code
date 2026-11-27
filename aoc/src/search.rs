//! Graph searches over any state type: give a start state and a function for its neighbours.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;

use rustc_hash::FxHashMap;

/// The fewest steps from `start` to every reachable state, by breadth-first search.
pub fn bfs<S, I>(start: S, mut next: impl FnMut(&S) -> I) -> FxHashMap<S, usize>
where
    S: Clone + Eq + Hash,
    I: IntoIterator<Item = S>,
{
    let mut dist = FxHashMap::default();
    dist.insert(start.clone(), 0);
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((s, d)) = queue.pop_front() {
        for n in next(&s) {
            // Look up before inserting, so only new states are cloned.
            if !dist.contains_key(&n) {
                dist.insert(n.clone(), d + 1);
                queue.push_back((n, d + 1));
            }
        }
    }
    dist
}

/// The cheapest cost from `start` to the first state where `goal` holds, and that state.
/// `next` gives each neighbour with the cost of the step to it.
pub fn dijkstra<S, I>(
    start: S,
    next: impl FnMut(&S) -> I,
    goal: impl FnMut(&S) -> bool,
) -> Option<(u64, S)>
where
    S: Clone + Eq + Hash + Ord,
    I: IntoIterator<Item = (S, u64)>,
{
    run_dijkstra(start, next, goal).1
}

/// The cheapest cost from `start` to every reachable state.
pub fn dijkstra_all<S, I>(start: S, next: impl FnMut(&S) -> I) -> FxHashMap<S, u64>
where
    S: Clone + Eq + Hash + Ord,
    I: IntoIterator<Item = (S, u64)>,
{
    run_dijkstra(start, next, |_| false).0
}

fn run_dijkstra<S, I>(
    start: S,
    mut next: impl FnMut(&S) -> I,
    mut goal: impl FnMut(&S) -> bool,
) -> (FxHashMap<S, u64>, Option<(u64, S)>)
where
    S: Clone + Eq + Hash + Ord,
    I: IntoIterator<Item = (S, u64)>,
{
    let mut dist = FxHashMap::default();
    dist.insert(start.clone(), 0);
    let mut heap = BinaryHeap::from([Reverse((0, start))]);
    while let Some(Reverse((d, s))) = heap.pop() {
        if d > dist[&s] {
            continue;
        }
        if goal(&s) {
            return (dist, Some((d, s)));
        }
        for (n, step) in next(&s) {
            let nd = d + step;
            if dist.get(&n).is_none_or(|&old| nd < old) {
                dist.insert(n.clone(), nd);
                heap.push(Reverse((nd, n)));
            }
        }
    }
    (dist, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_counts_steps_on_a_line() {
        let dist = bfs(0, |&n: &i32| {
            [n - 1, n + 1].into_iter().filter(|m| (0..5).contains(m))
        });
        assert_eq!(dist.len(), 5);
        assert_eq!(dist[&4], 4);
    }

    #[test]
    fn dijkstra_prefers_cheap_detour() {
        // 0 -> 2 costs 10 directly, or 1 + 1 through 1.
        let edges = |&n: &u8| match n {
            0 => vec![(1, 1), (2, 10)],
            1 => vec![(2, 1)],
            _ => vec![],
        };
        assert_eq!(dijkstra(0, edges, |&n| n == 2), Some((2, 2)));
        assert_eq!(dijkstra(0, edges, |&n| n == 9), None);
        assert_eq!(dijkstra_all(0, edges)[&2], 2);
    }
}
