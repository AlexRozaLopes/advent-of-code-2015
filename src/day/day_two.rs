use crate::model::rectangle::Rectangle;
use std::iter::Map;
use std::str::Lines;

pub fn square_feet_of_paper(input: &str) -> i32 {
    list_of_rectangle(input)
        .map(|r| r.surface_area() + r.shortest_area())
        .sum()
}

fn list_of_rectangle(input: &str) -> Map<Lines<'_>, fn(&str) -> Rectangle> {
    input.lines().map(|line| {
        let dims: Vec<i32> = line.split('x').map(|n| n.parse::<i32>().unwrap()).collect();
        Rectangle::new(dims[0], dims[1], dims[2])
    })
}

pub fn feet_of_ribbon(input: &str) -> i32 {
    list_of_rectangle(input)
        .map(|r| r.shortest_distance_around_sides() + r.cubic_volume())
        .sum()
}
