use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn max_allowed(self) -> i64 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

fn parse_item(item: &str) -> (i64, Color) {
    let parts: Vec<&str> = item.trim().split_whitespace().collect();

    let count = i64::from_str(parts[0]).unwrap_or(0);

    let color = match parts.last() {
        Some(&"red") => Color::Red,
        Some(&"green") => Color::Green,
        Some(&"blue") => Color::Blue,
        _ => Color::Red,
    };

    (count, color)
}

fn parse_line(line: &str) -> (i64, Vec<(i64, Color)>) {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    let id = i64::from_str(parts[0].trim_start_matches("Game").trim().trim_start()).unwrap_or(0);
    let items = parts[1]
        .split([';', ','])
        .map(|item| parse_item(item))
        .collect();
    (id, items)
}

pub fn solve_part1() -> i64 {
    let input = fs::read_to_string("input/day2.txt").expect("Error reading file");

    let mut possible_games = HashSet::new();
    let mut game_id = 0;

    let _total_id: i64 = input
        .lines()
        .enumerate()
        .filter(|&(line_id, line)| {
            let (id, items) = parse_line(line);

            let impossible = items
                .iter()
                .any(|&(count, color)| count > color.max_allowed());

            if !impossible {
                possible_games.insert(id);
                game_id += 1;
                // println!("Game {} is possible", id);
            }

            !impossible && line_id == 0
        })
        .map(|_| 1)
        .sum();

    let result = possible_games.iter().sum::<i64>();

    // println!("Total ID: {}", result);

    result
}

pub fn solve_part2() -> i64 {
    let input = fs::read_to_string("input/day2_part2.txt").expect("Error reading file");

    let mut total_power = 0;

    for line in input.lines() {
        let color_counts = parse_line(line).1;

        let mut max_powers = (0, 0, 0);

        for (count, color) in color_counts.iter() {
            match color {
                Color::Red => max_powers.0 = max_powers.0.max(*count),
                Color::Green => max_powers.1 = max_powers.1.max(*count),
                Color::Blue => max_powers.2 = max_powers.2.max(*count),
            }
        }

        let product = max_powers.0 * max_powers.1 * max_powers.2;
        total_power += product;
    }

    total_power
}
