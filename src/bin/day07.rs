use std::fs;

fn get_input() -> Vec<i32> {
    let input = fs::read_to_string("./input/day07.txt").unwrap();

    input
        .split(",")
        .filter_map(|v| v.parse().ok())
        .collect()
}

pub fn challenge_seven_part_one() {
    let mut positions = get_input();
    positions.sort();

    let median = positions[positions.len() / 2];

    let result: i32 = positions
        .into_iter()
        .map(|p| (p - median).abs())
        .sum();

    println!("Challenge seven part one: {}", result)
}

pub fn challenge_seven_part_two() {
    let positions = get_input();
    let mut cheapest = 0;

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    for i in min..=max {
        let mut fuel = 0;
        for pos in positions.iter().copied() {
            let diff = (pos - i).abs();
            fuel += (diff * (diff + 1)) / 2;
        }

        if cheapest == 0 || fuel < cheapest {
            cheapest = fuel;
        }
    }

    println!("Challenge seven part two: {}", cheapest)
}

fn main() {
    challenge_seven_part_one();
    challenge_seven_part_two();
}