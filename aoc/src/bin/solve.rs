//! `cargo solve [year] day`: run that day's solution in release mode.

use std::process::{Command, exit};

use aoc::tools::puzzle_from_args;
use aoc::{bin_name, package_name};

fn main() {
    let (year, day) = puzzle_from_args();
    let status = Command::new(env!("CARGO"))
        .args(["run", "--release", "--quiet"])
        .args(["--package", &package_name(year), "--bin", &bin_name(day)])
        .status()
        .expect("cannot start cargo");
    exit(status.code().unwrap_or(1));
}
