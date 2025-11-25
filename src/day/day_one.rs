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
