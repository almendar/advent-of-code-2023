use std::collections::HashMap;
use std::ops::Range;

use crate::common::load_input_map;

pub fn adjacent(y: usize, x1: usize, x2: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut acc: Vec<(usize, usize)> = vec![];
    let x_range: Range<usize> = x1..x2 + 1;
    let y_start = std::cmp::max(0, y as i32 - 1) as usize;
    let y_end = std::cmp::min(y + 1, max_y);
    let x_start = std::cmp::max(0, x1 as i32 - 1) as usize;
    let x_end = std::cmp::min(max_x, x2 + 1);
    for yi in y_start..y_end + 1 {
        for xi in x_start..x_end + 1 {
            if yi == y && x_range.contains(&xi) {
                continue;
            }
            acc.push((xi, yi))
        }
    }
    acc
}

struct Number {
    y: usize,
    x1: usize,
    x2: usize, // like Range, open on right.
}

fn find_numbers(board: &Vec<String>) -> Vec<Number> {
    let mut acc = vec![];
    for (y, line) in board.iter().enumerate() {
        let indexable = line.as_bytes();
        let mut row_idx = 0;
        while row_idx < line.len() {
            if indexable[row_idx].is_ascii_digit() {
                let x1 = row_idx;
                row_idx += 1;
                while row_idx < line.len() && indexable[row_idx].is_ascii_digit() {
                    row_idx += 1
                }
                let x2 = row_idx - 1;
                acc.push(Number { y, x1, x2 })
            }
            row_idx += 1;
        }
    }
    acc
}

pub fn part1(input: &str) -> u64 {
    let board = load_input_map(input).expect("This should load");
    let max_y = board.len() - 1;
    let max_x = board[0].len() - 1;

    find_numbers(&board)
        .iter()
        .filter_map(|number| {
            let adj = adjacent(number.y, number.x1, number.x2, max_x, max_y);
            let is_engine_part = adj.iter().any(|x| {
                let symbol = board[x.1].as_bytes()[x.0];
                symbol != b'.' && !symbol.is_ascii_digit()
            });
            if is_engine_part {
                let k = &board[number.y][number.x1..number.x2 + 1];
                Some(k.parse::<u64>().expect("not a valid number"))
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let board = load_input_map(input).expect("This should load");
    let max_y = board.len() - 1;
    let max_x = board[0].len() - 1;

    let mut engine: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    for number in find_numbers(&board) {
        for adj in adjacent(number.y, number.x1, number.x2, max_x, max_y) {
            let symbol = board[adj.1].as_bytes()[adj.0];
            if symbol == b'*' {
                let number = &board[number.y][number.x1..number.x2 + 1]
                    .parse::<u64>()
                    .expect("not a valid number");
                engine.entry(adj).or_insert_with(Vec::new).push(*number)
            }
        }
    }
    engine
        .iter()
        .filter_map(|x| {
            if x.1.len() == 2 {
                Some(x.1.iter().product::<u64>())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day3() {
        let part1_sample = part1("input/day3_sample.txt");
        assert_eq!(4361, part1_sample);
        let part1_input = part1("input/day3_input.txt");
        assert_eq!(522726, part1_input);
        println!("day3 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day3_sample.txt");
        assert_eq!(467835, part2_sample);
        let part2_input = part2("input/day3_input.txt");
        assert_eq!(81721933, part2_input);
        println!("day3 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}
