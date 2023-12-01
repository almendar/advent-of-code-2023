use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn find_first_last(line: &str) -> (i32, i32) {
    let mut first = -1;
    let mut last = -1;

    for c in line.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as i32;
            if first == -1 {
                first = digit;
            }
            last = digit;
        }
    }
    (first, last)
}

fn find_with_prefixes_first_last(line: &str, prefixes: &HashMap<String, i32>) -> (i32, i32) {
    let mut first = -1;
    let mut last = -1;

    let mut chars = line.char_indices().peekable();

    while let Some((start, c)) = chars.peek().copied() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as i32;
            if first == -1 {
                first = digit;
            }
            last = digit;
            chars.next();
            continue;
        }

        for (prefix, prefix_value) in prefixes.iter() {
            if line[start..].starts_with(prefix) {
                if first == -1 {
                    first = *prefix_value;
                }
                last = *prefix_value;
                break;
            }
        }
        chars.next();
    }
    if last == -1 {
        last = first;
    }

    (first, last)
}

pub fn task1(input: &str) -> io::Result<i32> {
    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    for line in reader.lines() {
        let (first, last) = find_first_last(&line?);
        counter += first * 10 + last;
    }

    Ok(counter)
}

pub fn task2(input: &str) -> io::Result<i32> {
    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut prefixes: HashMap<String, i32> = HashMap::new();
    prefixes.insert("one".to_owned(), 1);
    prefixes.insert("two".to_owned(), 2);
    prefixes.insert("three".to_owned(), 3);
    prefixes.insert("four".to_owned(), 4);
    prefixes.insert("five".to_owned(), 5);
    prefixes.insert("six".to_owned(), 6);
    prefixes.insert("seven".to_owned(), 7);
    prefixes.insert("eight".to_owned(), 8);
    prefixes.insert("nine".to_owned(), 9);

    let mut counter = 0;
    for line in reader.lines() {
        let (first, last) = find_with_prefixes_first_last(&line?, &prefixes);
        counter += first * 10 + last;
    }
    Ok(counter)
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
