//! `cargo submit [year] [day] part answer`: send an answer and print the site's reply.
//! The year and day work as for `new-day`; with neither, the puzzle is today's.

use aoc::tools::{first_element_text, puzzle_from, request};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [puzzle @ .., part, answer] = &args[..] else {
        panic!("expected `[year] [day] part answer`");
    };
    assert!(
        part == "1" || part == "2",
        "part must be 1 or 2, not `{part}`"
    );
    let (year, day) = puzzle_from(puzzle);
    let form = [("level", part.as_str()), ("answer", answer.as_str())];
    let reply = request(&format!("{year}/day/{day}/answer"), &form)
        .unwrap_or_else(|e| panic!("{e} (has the session cookie expired?)"));
    let text = first_element_text(&reply, "article").unwrap_or(reply);
    println!("{year} day {day} part {part}: {}", text.trim());
}
