use std::collections::HashSet;

pub fn how_many_houses(input: &str) -> usize {
    let mut position = (0, 0);
    let mut houses = HashSet::new();
    input.chars().for_each(|p| {
        match p {
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            _ => {}
        }
        houses.insert(position);
    });

    houses.len()
}

pub fn how_many_houses_with_robot_santa(input: &str) -> usize {
    let mut santa = (0, 0);
    let mut bot = (0, 0);

    let mut houses = HashSet::new();

    input.chars().enumerate().for_each(|(i, p)| {
        if i%2==0 {
            match p {
                '^' => santa.1 += 1,
                'v' => santa.1 -= 1,
                '>' => santa.0 += 1,
                '<' => santa.0 -= 1,
                _ => {}
            }
            houses.insert(santa);
        } else {
            match p {
                '^' => bot.1 += 1,
                'v' => bot.1 -= 1,
                '>' => bot.0 += 1,
                '<' => bot.0 -= 1,
                _ => {}
            }
            houses.insert(bot);
        }
    });

    houses.len()
}
