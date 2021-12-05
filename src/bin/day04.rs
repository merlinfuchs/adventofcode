use std::collections::HashMap;
use std::fs;

fn get_input() -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let input = fs::read_to_string("./input/day04.txt").unwrap();

    let sections: Vec<&str> = input
        .split("\n\n")
        .filter(|v| !v.is_empty())
        .collect();

    let moves = sections[0]
        .split(",")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let fields = sections[1..]
        .into_iter()
        .map(|f| f
            .split("\n")
            .filter(|r| !r.is_empty())
            .map(|r| r
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
            )
            .collect()
        )
        .collect();

    (moves, fields)
}

pub fn get_field_score(moves: &[usize], field: &Vec<Vec<usize>>) -> usize {
    let field_sum = field
        .iter()
        .map(|r| r
            .iter()
            .filter(|v| !moves.contains(v))
            .fold(0, |a, b| a + b)
        )
        .fold(0, |a, b| a + b);

    let last_move = moves[moves.len() - 1];
    last_move * field_sum
}

pub fn get_field_scores(moves: Vec<usize>, fields: Vec<Vec<Vec<usize>>>, count: usize) -> HashMap<usize, (usize, usize)> {
    let mut result = HashMap::new();

    for m in 0..moves.len() {
        let current_moves = &moves[0..m];

        for f in 0..fields.len() {
            if result.contains_key(&f) {
                continue;
            }

            let field = &fields[f];
            for row in field {
                if row.iter().all(|v| current_moves.contains(v)) {
                    let score = get_field_score(current_moves, field);
                    result.insert(f, (m + 1, score));
                    break;
                }
            }

            for col in 0..field[0].len() {
                let col: Vec<usize> = field
                    .iter()
                    .map(|r| r[col])
                    .collect();

                if col.iter().all(|v| current_moves.contains(v)) {
                    let score = get_field_score(current_moves, field);
                    result.insert(f, (m + 1, score));
                    break;
                }
            }
        }

        if result.len() >= count {
            break;
        }
    }

    result
}

pub fn challenge_four_part_one() {
    let (moves, fields) = get_input();
    let scores = get_field_scores(moves, fields, 1);

    let (field, (move_count, score)) = scores.iter().next().unwrap();
    println!("Challenge four part one: board {} after {} moves with a score of {}", field, move_count, score)
}

pub fn challenge_four_part_two() {
    let (moves, fields) = get_input();

    let field_count = fields.len();
    let scores = get_field_scores(moves, fields, field_count);
    let mut sorted_scores: Vec<(usize, (usize, usize))> = scores
        .into_iter()
        .collect();
    sorted_scores.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    let (field, (move_count, score)) = sorted_scores[sorted_scores.len() - 1];
    println!("Challenge four part one: board {} after {} moves with a score of {}", field, move_count, score)
}

fn main() {
    challenge_four_part_one();
    challenge_four_part_two();
}
