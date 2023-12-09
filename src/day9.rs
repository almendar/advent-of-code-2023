use crate::common::fold_on_each_line1;

fn parse(input: &str) -> Vec<Vec<i64>> {
    fold_on_each_line1(input, |line| {
        line.trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().expect("wrong pargins"))
            .collect()
    })
    .expect("wrong input")
}

fn expand(numbers: &Vec<i64>) -> Vec<Vec<i64>> {
    let shrink: Vec<i64> = numbers.windows(2).map(|x| x[1] - x[0]).collect();
    if shrink.iter().all(|x| *x == 0) {
        vec![shrink]
    } else {
        let mut k = expand(&shrink);
        k.push(shrink);
        k
    }
}

pub fn part1(input: &str) -> i64 {
    let input = parse(input);
    let k: Vec<i64> = input
        .iter()
        .map(|x| {
            let mut expanded = expand(x);
            expanded.push(x.clone());
            expanded
                .iter()
                .skip(1)
                .fold(0, |acc, el| el.last().unwrap() + acc)
        })
        .collect();
    k.iter().sum()
}

pub fn part2(input: &str) -> i64 {
    let input = parse(input);
    let k: Vec<i64> = input
        .iter()
        .map(|x| {
            let mut expanded = expand(x);
            expanded.push(x.clone());
            expanded
                .iter()
                .skip(1)
                .fold(0, |acc, el| el.first().unwrap() - acc)
        })
        .collect();
    k.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day9() {
        let part1_sample = part1("input/day9_sample.txt");
        assert_eq!(114, part1_sample);
        let part1_input = part1("input/day9_input.txt");
        assert_eq!(1581679977, part1_input);
        println!("day9 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day9_sample.txt");
        assert_eq!(2, part2_sample);
        let part2_input = part2("input/day9_input.txt");
        assert_eq!(889, part2_input);
        println!("day9 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}
