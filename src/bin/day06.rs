use std::fs;

fn get_input() -> Vec<u8> {
    let input = fs::read_to_string("./input/day06.txt").unwrap();

    input
        .split(",")
        .filter_map(|v| v.parse().ok())
        .collect()
}

pub fn old_simulate_fish_population(fish: &[u8], days: usize) -> usize {
    let mut vec = fish.to_vec();

    for _ in 0..days {
        for i in 0..vec.len() {
            let val = vec[i];
            if val == 0 {
                vec[i] = 6;
                vec.push(8);
            } else {
                vec[i] -= 1;
            }
        }
    }

    return vec.len();
}

pub fn simulate_fish_population(fish: &[u8], days: usize) -> usize {
    let mut counts = [0usize; 9];

    for v in fish {
        counts[*v as usize] += 1;
    }

    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.iter().sum()
}

pub fn challenge_six_part_one() {
    let mut fish = get_input();
    let result = simulate_fish_population(&fish, 80);

    println!("Challenge six part one: {}", result)
}

pub fn challenge_six_part_two() {
    let mut fish = get_input();
    let result = old_simulate_fish_population(&fish, 256);

    println!("Challenge six part two: {}", result)
}

fn main() {
    challenge_six_part_one();
    challenge_six_part_two();
}