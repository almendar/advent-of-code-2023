use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn find_with_prefixes_first_last(line: String) -> i32 {
    let prefixes: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let left_right: (Option<i32>, Option<i32>) =
        line.char_indices()
            .fold((Option::None, Option::None), |el, item| {
                let mut what_we_found: Option<i32> = None;

                if item.1.is_digit(10) {
                    what_we_found = item.1.to_digit(10).map(|x| x as i32);
                };

                for (k, v) in &prefixes {
                    if line[item.0..].starts_with(k) {
                        what_we_found = Some(*v as i32);
                    }
                }
                if let Some(number) = what_we_found {
                    match el {
                        (Some(fst), _) => (Some(fst), Some(number)),
                        (None, None) => (Some(number), Some(number)),
                        (None, Some(_)) => panic!("This is impossible!"),
                    }
                } else {
                    el
                }
            });

    left_right.0.unwrap() * 10 + left_right.1.unwrap()
}

fn find_first_last(line: String) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|x| x.to_digit(10)).collect();
    *digits.first().unwrap() * 10 + *digits.last().unwrap()
}

pub fn task1(input: &str) -> io::Result<u32> {
    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .map(find_first_last)
        .sum())
}

pub fn task2(input: &str) -> io::Result<i32> {
    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .map(find_with_prefixes_first_last)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let task1_example_result = task1("input/day1_example.txt").unwrap();
        assert_eq!(task1_example_result, 142);
        // println!("Result for day1_example.txt: {}", result1);

        let task1_result = task1("input/day1_input.txt").unwrap();
        assert_eq!(task1_result, 54990);

        let task2_example_result = task2("input/day1_example_t2.txt").unwrap();
        assert_eq!(task2_example_result, 281);

        let task2_result = task2("input/day1_input.txt").unwrap();
        assert_eq!(task2_result, 54473)
    }
}