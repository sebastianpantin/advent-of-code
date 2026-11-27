use aoc::*;

fn part1(input: &str) -> i64 {
    input.lines().count() as i64
}

fn part2(input: &str) -> i64 {
    input.lines().count() as i64
}

fn main() {
    let input = read_input!();
    timed("Part 1", || part1(&input));
    timed("Part 2", || part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"EXAMPLE_TEXT"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
