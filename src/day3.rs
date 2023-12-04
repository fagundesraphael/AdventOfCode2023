use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn is_char_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn get(contents: &Vec<Vec<char>>, i: usize, j: usize, offset: (i32, i32)) -> Option<char> {
    let chars = contents.get((i as i32 + offset.0) as usize)?;
    chars.get((j as i32 + offset.1) as usize).cloned()
}

pub fn solve_part1() -> i32 {
    let contents = fs::read_to_string("input/day3.txt").expect("Couldn't read input file");
    let contents: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;

    for y in 0..contents.len() {
        let row = &contents[y];
        let mut is_number = false;
        let mut current_number = String::new();
        let mut check = true;

        for x in 0..row.len() {
            is_number = is_char_number(get(&contents, y, x, (0, 0)).unwrap_or('\0'));

            if !is_number && !check {
                sum += current_number.parse::<i32>().unwrap_or(0);
            }

            if !is_number {
                current_number.clear();
                check = true;
            }

            if is_number && check {
                let is = DIRECTIONS.iter().any(|&(dy, dx)| {
                    let c = get(&contents, y, x, (dy, dx)).unwrap_or('\0');
                    !is_dot(c) && !is_char_number(c) && c != '\0'
                });

                if is {
                    check = false;
                }
            }

            if is_number {
                current_number.push(get(&contents, y, x, (0, 0)).unwrap_or('\0'));
            }
        }

        if is_number && !check {
            sum += current_number.parse::<i32>().unwrap_or(0);
        }
    }

    sum
}
