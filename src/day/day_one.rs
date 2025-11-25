pub fn what_floor(input: &str) -> i32 {
    input.chars().fold(0, |floor, c| {
        floor
            + match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
    })
}

pub fn first_basement(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .scan(0, |floor, (i, c)| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            Some((i, *floor))
        })
        .find(|(_, floor)| *floor == -1)
        .map(|(i, _)| i + 1).unwrap()
}
