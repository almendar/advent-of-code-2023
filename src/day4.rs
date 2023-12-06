use crate::common::fold_on_each_line1;
use std::collections::HashSet;

#[derive(Debug)]
struct ScratchCard {
    win_number: HashSet<u64>,
    my_numbers: HashSet<u64>,
}

impl ScratchCard {
    fn intersection_size(self) -> usize {
        self.win_number.intersection(&self.my_numbers).count()
    }
}

pub fn parse_to_vec(input: &str) -> HashSet<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|x| {
            x.trim()
                .parse::<u64>()
                .expect("should parse wins as number")
        })
        .collect()
}

fn folder(line: String) -> ScratchCard {
    let lr: Vec<&str> = line.trim().split(":").collect();
    if lr.len() != 2 {
        panic!("Expected the line to split at two")
    }
    let numbers: Vec<&str> = lr[1].trim().split("|").collect();
    if numbers.len() != 2 {
        panic!("Expected to have line split at two")
    }
    let wins = parse_to_vec(numbers[0]);
    let mu_numbers = parse_to_vec(numbers[1]);

    ScratchCard {
        win_number: wins,
        my_numbers: mu_numbers,
    }
}

pub fn part1(input: &str) -> u64 {
    let cards = fold_on_each_line1(input, folder).expect("Should be able to read");
    cards
        .into_iter()
        .map(|card| match card.intersection_size() {
            0 => 0,
            other => 1 << (other as i64 - 1),
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let cards = fold_on_each_line1(input, folder).expect("Should be able to read");
    let mut p = vec![1; cards.len()];
    for (index, card) in cards.into_iter().enumerate() {
        let matching = card.intersection_size();
        for i in index + 1..index + 1 + matching {
            p[i] += p[index]
        }
    }
    p.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day4() {
        let part1_sample = part1("input/day4_sample.txt");
        assert_eq!(13, part1_sample);
        let part1_input = part1("input/day4_input.txt");
        assert_eq!(20117, part1_input);
        println!("day4 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day4_sample.txt");
        assert_eq!(30, part2_sample);
        let part2_input = part2("input/day4_input.txt");
        assert_eq!(13768818, part2_input);
        println!("day4 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}
