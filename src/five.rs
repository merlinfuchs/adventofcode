use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn get_points(&self, count_diagonals: bool) -> Vec<Point> {
        let mut points = vec![];

        if !count_diagonals && self.a.y != self.b.y && self.a.x != self.b.x {
            return points;
        }

        let mut curr_x = self.a.x;
        let mut curr_y = self.a.y;

        points.push(Point {
            x: curr_x,
            y: curr_y,
        });

        while curr_x != self.b.x || curr_y != self.b.y {
            if curr_x < self.b.x {
                curr_x += 1;
            } else if curr_x > self.b.x {
                curr_x -= 1;
            }

            if curr_y < self.b.y {
                curr_y += 1;
            } else if curr_y > self.b.y {
                curr_y -= 1;
            }

            points.push(Point {
                x: curr_x,
                y: curr_y,
            });
        }

        points
    }
}

fn get_input() -> Vec<Line> {
    let input = fs::read_to_string("./input/five.txt").unwrap();

    let raw_lines: Vec<&str> = input
        .split("\n")
        .filter(|v| !v.is_empty())
        .collect();

    let mut result = vec![];
    for raw_line in raw_lines {
        let line: Vec<Vec<usize>> = raw_line
            .split(" -> ")
            .map(|s| s
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
            )
            .collect();

        result.push(Line {
            a: Point {
                x: line[0][0],
                y: line[0][1],
            },
            b: Point {
                x: line[1][0],
                y: line[1][1],
            },
        })
    }

    result
}

pub fn challenge_five_part_one() {
    let lines = get_input();
    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines {
        for point in line.get_points(false) {
            let key = (point.x, point.y);
            let current = *points.get(&key).unwrap_or(&0);
            points.insert(key, current + 1);
        }
    }

    let count = points
        .into_iter()
        .filter(|v| v.1 >= 2)
        .count();

    println!("Challenge five part one: {}", count)
}

pub fn challenge_five_part_two() {
    let lines = get_input();
    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines {
        for point in line.get_points(true) {
            let key = (point.x, point.y);
            let current = *points.get(&key).unwrap_or(&0);
            points.insert(key, current + 1);
        }
    }

    let count = points
        .into_iter()
        .filter(|v| v.1 >= 2)
        .count();

    println!("Challenge five part two: {}", count)
}

pub fn challenge_five() {
    challenge_five_part_one();
    challenge_five_part_two();
}