use crate::model::rectangle::Rectangle;

pub fn square_feet_of_paper(input: &str) -> i32 {
    let rec = input.lines()
        .collect::<Vec<&str>>()
        .iter().map(|l| {
        let vec = l.split("x").collect::<Vec<&str>>();
        Rectangle::new(vec.get(0).unwrap().parse().unwrap(),
                       vec.get(1).unwrap().parse().unwrap(),
                       vec.get(2).unwrap().parse().unwrap(),
        )
    }).collect::<Vec<Rectangle>>();

    rec.iter().fold(0, |acc, r| {
        acc + r.surface_area() + r.shortest_area()
    })
}