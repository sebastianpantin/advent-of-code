//! Support for the `new-day` and `solve` tools: settings and choosing the puzzle.

use std::time::{SystemTime, UNIX_EPOCH};

/// A value from the environment, or else from a `KEY=value` line in the workspace `.env`.
pub fn setting(key: &str) -> Option<String> {
    std::env::var(key).ok().or_else(|| {
        let env_file = std::fs::read_to_string(crate::workspace_root().join(".env")).ok()?;
        env_file.lines().find_map(|line| {
            let (k, v) = line.split_once('=')?;
            (k.trim() == key).then(|| v.trim().trim_matches('"').to_string())
        })
    })
}

/// The puzzle chosen by the command-line arguments `[year] [day]`.
///
/// - `year day`: that puzzle.
/// - `day`: that day of `AOC_YEAR`, or of the latest event if `AOC_YEAR` is not set.
/// - nothing: today's puzzle, during December.
pub fn puzzle_from_args() -> (u32, u32) {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .map(|a| {
            a.parse()
                .unwrap_or_else(|_| panic!("`{a}` is not a number"))
        })
        .collect();
    let (year, month, today) = today_est();
    let (year, day) = match args[..] {
        [year, day] => (year, day),
        [day] => (default_year(year, month), day),
        [] if month == 12 => (year, today),
        [] => panic!("not December; pass the day, e.g. `1` or `2026 1`"),
        _ => panic!("expected `[year] [day]`"),
    };
    assert!(year >= 2015, "Advent of Code started in 2015");
    assert!((1..=25).contains(&day), "day must be 1 to 25");
    (year, day)
}

fn default_year(year: u32, month: u32) -> u32 {
    match setting("AOC_YEAR") {
        Some(y) => y.parse().expect("AOC_YEAR must be a year"),
        None if month == 12 => year,
        None => year - 1,
    }
}

/// Today as (year, month, day) on the puzzle release clock (EST, UTC-5).
fn today_est() -> (u32, u32, u32) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    civil_from_days((secs - 5 * 3600).div_euclid(86_400))
}

/// Days since 1970-01-01 to (year, month, day), from Howard Hinnant's `civil_from_days`.
fn civil_from_days(days: i64) -> (u32, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = (yoe + era * 400) as u32 + u32::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::civil_from_days;

    #[test]
    fn civil_from_days_matches_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(20_787), (2026, 11, 30));
        assert_eq!(civil_from_days(20_788), (2026, 12, 1));
        assert_eq!(civil_from_days(20_819), (2027, 1, 1));
    }
}
