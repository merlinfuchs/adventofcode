use std::fs;

fn get_input() -> (usize, Vec<usize>) {
    let input = fs::read_to_string("./input/day03.txt").unwrap();

    let mut bit_length = 0;
    let values = input
        .split("\n")
        .filter(|v| !v.is_empty())
        .map(|v| {
            bit_length = v.len();
            usize::from_str_radix(v, 2).unwrap()
        })
        .collect();

    (bit_length, values)
}

pub fn challenge_three_part_one() {
    let (bit_length, values) = get_input();

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in 0..bit_length {
        let bit_value = 1 << bit;

        let mut one_count = 0;
        for value in &values {
            if value & bit_value != 0 {
                one_count += 1;
            }
        }

        let zero_count = values.len() - one_count;
        if one_count > zero_count {
            gamma |= bit_value
        } else if zero_count > one_count {
            epsilon |= bit_value
        }
    }

    let result = gamma * epsilon;
    println!("Challenge three part one: {}", result)
}

pub fn challenge_three_part_two() {
    let (bit_length, values) = get_input();

    let mut oxygen = values.clone();
    let mut co2 = values.clone();

    for bit in 0..bit_length {
        let bit_value = 1 << bit;

        if oxygen.len() > 1 {
            let mut ones = vec![];
            let mut zeroes = vec![];

            for value in oxygen {
                if value & bit_value != 0 {
                    ones.push(value)
                } else {
                    zeroes.push(value)
                }
            }

            if ones.len() >= zeroes.len() {
                oxygen = ones;
            } else {
                oxygen = zeroes;
            }
        }

        if co2.len() > 1 {
            let mut ones = vec![];
            let mut zeroes = vec![];

            for value in co2 {
                if value & bit_value != 0 {
                    ones.push(value)
                } else {
                    zeroes.push(value)
                }
            }

            if ones.len() < zeroes.len() {
                co2 = ones;
            } else {
                co2 = zeroes;
            }
        }

        if oxygen.len() <= 1 && co2.len() <= 1 {
            break;
        }
    }

    let result = oxygen[0] * co2[0];
    println!("Challenge three part two: {}", result)
}

fn main() {
    challenge_three_part_one();
    challenge_three_part_two();
}
