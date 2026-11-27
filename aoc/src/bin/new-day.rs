//! `cargo new-day [year] [day]`: create `YEAR/src/bin/dayNN.rs` from the template and
//! fetch the puzzle input into `inputs/YEAR/dayNN.txt`. The first time a year is used,
//! its crate is made with `cargo new`, which also adds it to the workspace members.
//! Each step is skipped if its file already exists, so an input is requested only once.
//!
//! Needs `AOC_SESSION` (the `session` cookie from adventofcode.com), from the environment
//! or from `.env`.

use std::fs;
use std::path::Path;
use std::process::Command;

use aoc::tools::{puzzle_from_args, setting};
use aoc::{bin_name, input_path, package_name, workspace_root};

const TEMPLATE: &str = include_str!("../../../templates/day.rs");

fn main() {
    let (year, day) = puzzle_from_args();
    let crate_dir = workspace_root().join(year.to_string());
    if !crate_dir.join("Cargo.toml").exists() {
        create_year_crate(year, &crate_dir);
    }
    let source = crate_dir.join(format!("src/bin/{}.rs", bin_name(day)));
    create_once(&source, "created", || TEMPLATE.to_string());
    create_once(&input_path(year, day), "fetched", || fetch_input(year, day));
}

/// Make the `aocYEAR` library crate with Cargo itself, so Cargo edits the manifests.
/// Its `src/lib.rs` is the place for code shared by that year's days.
fn create_year_crate(year: u32, crate_dir: &Path) {
    let package = package_name(year);
    let dir = crate_dir.to_str().unwrap();
    cargo(&[
        "new", "--quiet", "--lib", "--vcs", "none", "--name", &package, dir,
    ]);
    let aoc = workspace_root().join("aoc");
    cargo(&[
        "add",
        "--quiet",
        "--package",
        &package,
        "--path",
        aoc.to_str().unwrap(),
    ]);
    fs::write(
        crate_dir.join("src/lib.rs"),
        format!("//! Code shared by the {year} days.\n"),
    )
    .unwrap();
    println!("created crate {package}");
}

fn cargo(args: &[&str]) {
    let status = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(workspace_root())
        .status()
        .expect("cannot start cargo");
    assert!(status.success(), "`cargo {}` failed", args.join(" "));
}

/// Write `path` with `contents()` unless it already exists.
fn create_once(path: &Path, verb: &str, contents: impl FnOnce() -> String) {
    let shown = path
        .strip_prefix(workspace_root())
        .unwrap_or(path)
        .display();
    if path.exists() {
        println!("{shown} already exists");
        return;
    }
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents()).unwrap();
    println!("{verb} {shown}");
}

fn fetch_input(year: u32, day: u32) -> String {
    let session = setting("AOC_SESSION").expect("set AOC_SESSION in .env");
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let output = Command::new("curl")
        .args(["--silent", "--show-error", "--fail"])
        .args(["--user-agent", "advent-of-code new-day (Rust, curl)"])
        .args(["--cookie", &format!("session={session}"), &url])
        .output()
        .expect("curl is needed to fetch inputs");
    assert!(
        output.status.success(),
        "fetching {url} failed: {}(the puzzle may not be unlocked yet, or the session cookie has expired)",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("input is not UTF-8")
}
