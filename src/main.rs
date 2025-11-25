mod day;

use crate::day::*;

fn main() {
    println!(
        "Day 1: Not Quite Lisp: PART ONE - ANSWER {}",
        day_one::what_floor(include_str!("data/day-one"))
    )
}
