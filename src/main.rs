mod day;

use crate::day::*;

fn main() {
    println!(
        "Day 1: Not Quite Lisp: PART ONE - ANSWER {}",
        day_one::what_floor(include_str!("data/day-one"))
    );

    println!(
            "Day 1: Not Quite Lisp: PART TWO - ANSWER {}",
        day_one::first_basement(include_str!("data/day-one"))
    )
}
