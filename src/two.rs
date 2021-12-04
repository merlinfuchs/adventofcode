use std::fs;

fn get_input() -> Vec<(String, usize)> {
    let input = fs::read_to_string("./input/two.txt").unwrap();
    input
        .split("\n")
        .filter(|v| !v.is_empty())
        .map(|v| v.split(" ").collect())
        .map(|v: Vec<&str>| (
            v[0].to_string(),
            v[1].parse::<usize>().unwrap()
        ))
        .collect()
}

pub fn challenge_two_part_one() {
    let values = get_input();

    let mut position = 0;
    let mut depth = 0;

    for (instruction, delta) in &values {
        match instruction.as_str() {
            "down" => depth += delta,
            "up" => depth -= delta,
            "forward" => position += delta,
            _ => {}
        }
    }

    let result = position * depth;
    println!("Challenge two part one: {}", result)
}

pub fn challenge_two_part_two() {
    let values = get_input();

    let mut aim = 0;
    let mut position = 0;
    let mut depth = 0;

    for (instruction, delta) in &values {
        match instruction.as_str() {
            "down" => aim += delta,
            "up" => aim -= delta,
            "forward" => {
                position += delta;
                depth += aim * delta;
            },
            _ => {}
        }
    }

    let result = position * depth;
    println!("Challenge two part two: {}", result)
}