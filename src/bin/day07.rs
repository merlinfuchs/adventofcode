use std::fs;

fn get_input() -> Vec<i32> {
    let input = fs::read_to_string("./input/day07.txt").unwrap();

    input
        .split(",")
        .filter_map(|v| v.parse().ok())
        .collect()
}

pub fn challenge_seven_part_one() {
    let positions = get_input();
    let mut cheapest = 0;

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    for i in min..=max {
        let mut fuel = 0;
        for pos in positions.iter().copied() {
            fuel += (pos - i).abs();
        }

        if cheapest == 0 || fuel < cheapest {
            cheapest = fuel;
        }
    }

    println!("Challenge seven part one: {}", cheapest)
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