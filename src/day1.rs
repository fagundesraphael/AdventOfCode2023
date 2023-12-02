use std::collections::HashMap;
use std::fs;

fn map_first_and_last_digits(line: &str) -> Option<u32> {
    let mut digits = line.chars().filter_map(|c| match c {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    });

    let first_digit = digits.next().unwrap_or(0);
    let second_digit = digits.last().unwrap_or(first_digit);

    Some(first_digit * 10 + second_digit)
}

pub fn solve_part1() -> u32 {
    let file = fs::read_to_string("input/day1.txt").expect("Couldn't read input file");

    let sum: u32 = file
        .lines()
        .filter_map(|line| map_first_and_last_digits(line))
        .sum();

    sum
}

pub fn solve_part2() -> u32 {
    let file = fs::read_to_string("input/day1_part2.txt").expect("Couldn't read input file");

    let sum: u32 = file
        .lines()
        .filter_map(|line| {
            let first_digit = map_word_to_number(line);
            let last_digit = map_word_to_number_reverse(line);

            match (first_digit, last_digit) {
                (Some(a), Some(b)) => {
                    let concat_number: u32 =
                        format!("{}{}", a, b).parse::<u32>().unwrap_or_default();
                    Some(concat_number)
                }
                _ => None,
            }
        })
        .sum();

    sum
}

fn apply_mapping(slice: &str) -> Option<String> {
    let number_as_word: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    for (key, value) in number_as_word.clone().into_iter() {
        if slice.contains(key) {
            return Some(value.to_string());
        }
    }
    None
}

fn map_word_to_number(line: &str) -> Option<String> {
    let mut number: Option<String> = None;

    for (i, current_character) in line.chars().enumerate() {
        if current_character.is_numeric() {
            number = Some(current_character.to_string());
            break;
        }

        if i >= 2 {
            let current_chars: String = line.chars().take(i + 1).collect::<String>();
            if let Some(mapped_number) = apply_mapping(&current_chars) {
                number = Some(mapped_number);
                break;
            }
        }
    }

    number
}

fn map_word_to_number_reverse(line: &str) -> Option<String> {
    for (i, current_character) in line.chars().rev().enumerate() {
        if current_character.is_numeric() {
            return Some(current_character.to_string());
        }

        if i >= 2 {
            let current_chars: String = line[line.len() - (i + 1)..].to_string();
            if let Some(mapped_number) = apply_mapping(&current_chars) {
                return Some(mapped_number);
            }
        }
    }

    None
}
