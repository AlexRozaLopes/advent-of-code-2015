use crate::model::rectangle::Rectangle;

pub fn square_feet_of_paper(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let dims: Vec<i32> = line
                .split('x')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            Rectangle::new(dims[0], dims[1], dims[2])
        })
        .map(|r| r.surface_area() + r.shortest_area())
        .sum()
}
