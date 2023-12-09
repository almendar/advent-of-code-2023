use std::collections::HashMap;

use crate::common::load_input_as_string;

fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let input = load_input_as_string(input).expect("loading failed");
    let input: Vec<_> = input.split("\n\n").collect();
    let instructions: Vec<char> = input[0].chars().collect();
    let graph: HashMap<_, _> = input[1]
        .lines()
        .map(|line| match line.split_once(" = ") {
            Some((from, to)) => match to[1..to.len() - 1].split_once(", ") {
                Some(instruction) => (
                    from.to_owned(),
                    (instruction.0.to_owned(), instruction.1.to_owned()),
                ),
                _ => panic!("nope"),
            },
            _ => panic!("Wrong parsing"),
        })
        .collect();
    (instructions.clone(), graph)
}

fn find_steps(
    start: &str,
    stop: &str,
    instructions: Vec<char>,
    graph: HashMap<String, (String, String)>,
) -> i64 {
    let mut current = start;
    let mut counter = 0;
    for i in instructions.iter().cycle() {
        if current == stop {
            break;
        }
        let (left, right) = graph.get(current).expect("whoaaa");
        match i {
            'L' => current = left,
            'R' => current = right,
            _ => panic!("lol"),
        }
        counter += 1
    }
    counter
}

pub fn part1(input: &str) -> i64 {
    let (instructions, graph) = parse(input);
    find_steps("AAA", "ZZZ", instructions, graph)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs() // Return the absolute value in case of negative inputs
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn lcm_list(numbers: &Vec<i64>) -> i64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

pub fn part2(input: &str) -> i64 {
    let (instructions, graph) = parse(input);
    let all_starting_points: Vec<_> = graph.keys().filter(|x| x.ends_with("A")).collect();
    let mut steps_per_start = vec![];
    for start in all_starting_points {
        let mut steps = 0;
        let mut current = start;
        for i in instructions.iter().cycle() {
            if current.ends_with("Z") {
                break;
            }
            steps += 1;
            let (left, right) = graph.get(current).expect("whoaaa");
            match i {
                'L' => current = left,
                'R' => current = right,
                _ => panic!("lol"),
            }
        }
        steps_per_start.push(steps);
    }
    lcm_list(&steps_per_start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day8() {
        let part1_sample = part1("input/day8_sample.txt");
        assert_eq!(6, part1_sample);
        let part1_input = part1("input/day8_input.txt");
        assert_eq!(15517, part1_input);
        println!("day8 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day8_sample.txt");
        assert_eq!(6, part2_sample);
        let part2_input = part2("input/day8_input.txt");
        assert_eq!(14935034899483, part2_input);
        println!("day8 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}
