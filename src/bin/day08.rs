use std::collections::{BTreeSet};
use std::fs;

fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    let input = fs::read_to_string("./input/day08.txt").unwrap();

    input
        .lines()
        .filter_map(|l| {
            let (signals, outputs) = l.split_once(" | ")?;
            Some((
                signals.split(" ").map(|v| v.to_string()).collect(),
                outputs.split(" ").map(|v| v.to_string()).collect()
            ))
        })
        .collect()
}

pub fn challenge_eight_part_one() {
    let input = get_input();
    let unique_lens = [2, 3, 4, 7];

    let unique_count: usize = input
        .into_iter()
        .map(|(_, o)| o
            .into_iter()
            .filter(|v| unique_lens.contains(&v.len()))
            .count()
        )
        .sum();

    println!("Challenge eight part one: {}", unique_count)
}

/*
signal lengths to character(s):

2 => 1
3 => 7
4 => 4
5 => 2 | 3 | 5
6 => 0 | 6 | 9
7 => 8
*/

pub fn decode_digits(signals: &Vec<String>) -> Vec<BTreeSet<u8>> {
    let mut result = vec![BTreeSet::new(); 10];

    // five and six are the only lenghts that aren't unique
    let mut sixes = vec![];
    let mut fives = vec![];

    for signal in signals {
        let digits: BTreeSet<u8> = signal.bytes().collect();

        match signal.len() {
            // add unique lengths to result
            2 => result[1] = digits,
            3 => result[7] = digits,
            4 => result[4] = digits,
            7 => result[8] = digits,
            // add other lengths to process later
            5 => fives.push(digits),
            6 => sixes.push(digits),
            _ => {}
        };
    }

    for digits in sixes {
        if !digits.is_superset(&result[1]) {
            // 6 is the only digit that doesn't contain 1
            result[6] = digits;
        } else if !digits.is_superset(&result[4]) {
            // 0 is the only digit left that doesn't contain 4
            result[0] = digits;
        } else {
            // 9 contains both and is the only digit left
            result[9] = digits;
        }
    }

    for digits in fives {
        if result[6].is_superset(&digits) {
            // 5 is the only digit that is contained by 6
            result[5] = digits;
        } else if result[9].is_superset(&digits) {
            // 3 is the only digit left that is contained by 9
            result[3] = digits;
        } else {
            // 2 is the only digit left
            result[2] = digits;
        }
    }

    result
}

pub fn challenge_eight_part_two() {
    let input = get_input();

    let result: usize = input
        .into_iter()
        .map(|(signals, outputs)| {
            let decoded = decode_digits(&signals);
            outputs
                .into_iter()
                .fold(0, |accum, output| {
                    let digits = output.bytes().collect();
                    let digit = decoded
                        .iter()
                        .position(|d| *d == digits)
                        .unwrap_or_default();

                    accum * 10 + digit
                })
        })
        .sum();

    println!("Challenge eight part one: {}", result)
}

fn main() {
    challenge_eight_part_one();
    challenge_eight_part_two();
}