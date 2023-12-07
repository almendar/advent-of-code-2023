// Time:      7  15   30
// Distance:  9  40  200

use crate::common::load_input_map;

fn load_races(input: &str) -> Vec<(i64, i64)> {
    let input = load_input_map(input).expect("loading task1 file");
    if input.len() != 2 {
        panic!("Wrong len of read data");
    }
    let times = match input[0].split_once(":") {
        Some((_, numbers)) => numbers
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap()),

        None => panic!("wrong split"),
    };

    let distance = match input[1].split_once(":") {
        Some((_, numbers)) => numbers
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap()),
        None => panic!("wrong split"),
    };

    times.zip(distance).collect()
}

pub fn part1(input: &str) -> i64 {
    let min_prod: i64 = load_races(input)
        .iter()
        .map(|race| {
            {
                let (time, distance) = race;
                (0..time + 1).filter(|t| (*time - t) * t > *distance)
            }
                .count() as i64
        })
        .product();

    min_prod
}

pub fn part2(input: &str) -> i64 {
    let races_pairs = load_races(input)
        .iter()
        .fold(("".to_owned(), "".to_owned()), |acc, (t, d)| {
            (format!("{}{}", acc.0, t), format!("{}{}", acc.1, d))
        });
    let time: i64 = races_pairs.0.parse().unwrap();
    let distance: i64 = races_pairs.1.parse().unwrap();
    (0..time + 1)
        .filter(|t| (time - t) * t > distance)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day6() {
        let part1_sample = part1("input/day6_sample.txt");
        assert_eq!(288, part1_sample);
        let part1_input = part1("input/day6_input.txt");

        assert_eq!(800280, part1_input);
        println!("day6 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day6_sample.txt");
        assert_eq!(71503, part2_sample);
        let part2_input = part2("input/day6_input.txt");
        assert_eq!(45128024, part2_input);
        println!("day6 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}
