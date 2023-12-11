use crate::common::Position;

use crate::common::load_input_map;

fn manhatan_dist(p1: &Position, p2: &Position) -> i64 {
    (p2.x.abs_diff(p1.x) + p2.y.abs_diff(p1.y))
        .try_into()
        .unwrap()
}

fn count_expands(g: &mut Vec<i64>, multiplier: i64) -> i64 {
    g.sort();
    let n = g.len() as i64;
    g.windows(2)
        .enumerate()
        .filter_map(|(i, w)| {
            let blanks = w[1] - w[0];
            if blanks > 1 {
                Some((n - (i as i64 + 1)) * (i as i64 + 1) * multiplier)
            } else {
                None
            }
        })
        .sum()
}

fn load_galaxies(input: &str) -> Vec<Position> {
    let map = load_input_map(input).expect("works?!");
    map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Position {
                        x: x as i64,
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let galaxies = load_galaxies(input);
    let mut res = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g_from)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|g_to| manhatan_dist(g_from, g_to))
        })
        .sum();

    let mut x_s: Vec<i64> = galaxies.iter().map(|x| x.x).collect();
    let mut y_s: Vec<i64> = galaxies.iter().map(|x| x.y).collect();
    res += count_expands(&mut x_s, 1);
    res += count_expands(&mut y_s, 1);
    res
}

pub fn part2(input: &str) -> i64 {
    let map = load_input_map(input).expect("works?!");
    let galaxies: Vec<Position> = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Position {
                        x: x as i64,
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    let mut res = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g_from)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|g_to| manhatan_dist(g_from, g_to))
        })
        .sum();

    let mut x_s: Vec<i64> = galaxies.iter().map(|x| x.x).collect();
    let mut y_s: Vec<i64> = galaxies.iter().map(|x| x.y).collect();

    if input.contains("sample") {
        res += count_expands(&mut x_s, 100 - 1);
        res += count_expands(&mut y_s, 100 - 1);
    } else {
        res += count_expands(&mut x_s, 1000000 - 1);
        res += count_expands(&mut y_s, 1000000 - 1);
    }


    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day11() {
        let part1_sample = part1("input/day11_sample.txt");
        assert_eq!(374, part1_sample);
        let part1_input = part1("input/day11_input.txt");
        assert_eq!(9521550, part1_input);
        println!(
            "day11 part1 sample:{}, input: {}",
            part1_sample, part1_input
        );
        let part2_sample = part2("input/day11_sample.txt");
        assert_eq!(8410, part2_sample);
        let part2_input = part2("input/day11_input.txt");
        assert_eq!(298932923702, part2_input);
        println!(
            "day11 part2 sample {}, input: {}", part2_sample, part2_input
        );
    }
}
