mod day;
mod model;

use crate::day::*;

fn main() {
    println!(
        "Day 1: Not Quite Lisp: PART ONE - ANSWER {}",
        day_one::what_floor(include_str!("data/day-one"))
    );

    println!(
        "Day 1: Not Quite Lisp: PART TWO - ANSWER {}",
        day_one::first_basement(include_str!("data/day-one"))
    );

    println!(
        "Day 2: I Was Told There Would Be No Math: PART ONE - ANSWER {}",
        day_two::square_feet_of_paper(include_str!("data/day-two"))
    );

    println!(
        "Day 2: I Was Told There Would Be No Math: PART TWO - ANSWER {}",
        day_two::feet_of_ribbon(include_str!("data/day-two"))
    )
}
