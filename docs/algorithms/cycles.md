# Cycles and number theory

## Cycle detection

**Use when:** a simulation must run a huge number of steps (a billion spins, a trillion rounds), and each step depends only on the current state.

**Idea:** if the state has a limited number of possible values, it must eventually repeat. Once a state repeats, everything after it repeats too, because the same state always leads to the same next state. So:

1. Run the simulation, storing each state with the step at which it was first seen.
2. When a state is seen a second time, at step `i`, and was first seen at step `first`, the cycle length is `i - first`.
3. The steps still to do are `TARGET - i`. Only the remainder `(TARGET - i) % cycle` matters, since whole cycles end where they started.
4. Run that many more steps. The state is now the one at `TARGET`.

```rust
fn state_after<S: Clone + Eq + std::hash::Hash>(start: S, target: usize, step: impl Fn(&S) -> S) -> S {
    let mut seen: FxHashMap<S, usize> = FxHashMap::default();
    let mut state = start;
    for i in 0..target {
        if let Some(&first) = seen.get(&state) {
            let left = (target - i) % (i - first);
            for _ in 0..left {
                state = step(&state);
            }
            return state;
        }
        seen.insert(state.clone(), i);
        state = step(&state);
    }
    state
}
```

`Grid` is `Hash + Eq`, so a whole grid can be the state.

**Variations:**

- **Store the history.** Keep the states in a `Vec` too. Then the answer is just `history[first + (target - first) % cycle]`, with no extra steps.
- **Key on less.** Sometimes the state that decides the future is smaller than the whole simulation. In a falling-rocks puzzle, the key is `(rock shape index, wind index, shape of the top few rows)`, while the height keeps growing. Store the height with each key and add `cycles * height_gained_per_cycle` at the end.
- **Do not confuse "the same value" with "the same state".** A score can repeat without the state repeating. Key on whatever decides the next step.

## Several cycles lining up (LCM)

**Use when:** several independent things each repeat with their own period, and you need the first moment they are all at the start together.

**Idea:** thing A is back at the start at multiples of `a`, thing B at multiples of `b`. Both are at the start at the common multiples, and the first one is the **least common multiple**, `lcm(a, b)`. For more than two, combine them one at a time.

```rust
let together = periods.iter().fold(1, |acc, &p| lcm(acc, p));
```

`lcm(a, b) = a / gcd(a, b) * b`. Dividing first keeps the numbers small.

**Check the input:** this only works if each thing truly loops from the start with a fixed period. Puzzle inputs are usually built so it does, but print the first few times each thing reaches its goal to confirm the gaps are equal. If each one first needs some steps before its loop begins, you need the Chinese remainder theorem instead.

## Chinese remainder theorem

**Use when:** you need the smallest `x` such that `x mod m1 = r1`, `x mod m2 = r2`, and so on. For example: buses leave every `m` minutes, and bus `i` must leave `i` minutes after `x`.

**Idea:** solve two conditions at a time. The `x` that meet both conditions are themselves all the numbers `≡ r (mod lcm(m1, m2))` for one `r`, so two conditions merge into one, and the next condition merges into that.

**Simple way (sieving):** to merge `x ≡ r (mod m)` with `x ≡ r2 (mod m2)`, try `r, r + m, r + 2m, ...` until one has remainder `r2`. That takes at most `m2` tries. Then the step becomes `lcm(m, m2)`. Fast whenever the moduli are small, which they usually are.

```rust
// conditions: (remainder, modulus) pairs. Returns the smallest x >= 0 meeting all of them.
fn crt(conditions: &[(i64, i64)]) -> i64 {
    let (mut x, mut step) = (0i64, 1i64);
    for &(r, m) in conditions {
        while x.rem_euclid(m) != r.rem_euclid(m) {
            x += step;
        }
        step = lcm(step, m);
    }
    x
}
```

`rem_euclid` gives a remainder from 0 to `m - 1`, also for negative numbers. Plain `%` can return negative values.

If the moduli are huge, use the extended Euclidean algorithm instead; see the link below. Use `i128` for the products if they can pass `i64::MAX` (about `9.2e18`).

## Modular arithmetic

When a puzzle shuffles a huge deck or works "mod" some number:

- `(a + b) % m`, `(a * b) % m` can be taken at every step, so numbers stay small.
- Products of two numbers below `m` can overflow `i64` if `m` is above about `3e9`. Cast to `i128` for the multiplication.
- Raising to a huge power: square-and-multiply, which needs about `log2(exponent)` steps.

```rust
fn pow_mod(mut base: i128, mut exp: u64, m: i128) -> i128 {
    let mut result = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}
```

## Further reading

- [Wikipedia: Cycle detection](https://en.wikipedia.org/wiki/Cycle_detection), including Floyd's tortoise-and-hare method, which uses no map.
- [CP-Algorithms: Chinese remainder theorem](https://cp-algorithms.com/algebra/chinese-remainder-theorem.html).
- [CP-Algorithms: Extended Euclidean algorithm](https://cp-algorithms.com/algebra/extended-euclid-algorithm.html).
