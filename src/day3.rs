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

fn part1(input: &str) -> u64 {
    let board = load_input_map(input).expect("This should load");
    let max_y = board.len()-1;
    let max_x = board[0].len()-1;
    let mut sum = 0;
    for (line_idx, line) in board.iter().enumerate() {
        let indexable = line.as_bytes();
        let mut row_idx = 0;
        while row_idx < line.len() {
            if indexable[row_idx].is_ascii_digit() {
                let start_number = row_idx;
                row_idx += 1;
                while (row_idx < line.len() && indexable[row_idx].is_ascii_digit()) {
                    row_idx += 1
                }
                let end_number = row_idx - 1;
                // println!("{} {}", start_number, end_number);
                let f = adjacent(line_idx, start_number, end_number, max_x, max_y);
                // println!("{:?}", f);

                let is_engine_part = f.iter().any(|x| {
                    let symbol = board[x.1].as_bytes()[x.0];
                    symbol != b'.' && !symbol.is_ascii_digit()
                });

                if is_engine_part {
                    let k = &line[start_number..end_number + 1];
                    let num: u64 = k.parse().expect("not a valid number");
                    sum += num
                }
            }
            row_idx += 1;
        }
    }
    sum
}


#[cfg(test)]
mod tests {
    use crate::day3::part1;
    #[test]
    fn run_day3() {
        let part1_sample = part1("input/day3_sample.txt");
        let part1_input = part1("input/day3_input.txt");
        println!("day3 part1 sample:{}, input: {}", part1_sample, part1_input);
    }
}
