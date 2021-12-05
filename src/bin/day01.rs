use std::fs;

fn get_input() -> Vec<usize> {
    let input = fs::read_to_string("./input/day01.txt").unwrap();
    input
        .split("\n")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

fn count_increases(values: &Vec<usize>, window_size: usize) -> usize {
    let mut result = 0;

    for i in 0..values.len() - window_size {
        let mut window_a = 0;
        for a in i..i + window_size {
            window_a += values[a];
        }

        let mut window_b = 0;
        for b in i + 1..i + window_size + 1 {
            window_b += values[b];
        }

        if window_b > window_a {
            result += 1;
        }
    }

    result
}

pub fn challenge_one_part_one() {
    let values = get_input();

    let result = count_increases(&values, 1);

    println!("Challenge one part one: {}", result)
}

pub fn challenge_one_part_two() {
    let values = get_input();

    let result = count_increases(&values, 3);

    println!("Challenge one part two: {}", result)
}

fn main() {
    challenge_one_part_one();
    challenge_one_part_two();
}
