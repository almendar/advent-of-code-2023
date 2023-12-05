use std::{fs, fs::File, io::{self, BufRead, BufReader}, path::Path};
use std::str::FromStr;

pub fn fold_on_each_line1<T, F>(input: &str, folder: F) -> io::Result<Vec<T>>
where
    F: Fn(String) -> T,
{
    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).map(folder).collect())
}

pub fn load_input_map(input: &str) -> io::Result<Vec<String>> {
    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|x| x.expect("Should be able to read line"))
        .collect::<Vec<String>>())
}

pub fn load_input_as_string(input: &str) -> io::Result<String> {
    fs::read_to_string(input)
}

pub(crate) fn parse_ints_separated_by_space<T>(input: &str) -> Result<Vec<T>, <T as FromStr>::Err>
    where
        T: FromStr,
{
    input.trim().split_whitespace().map(|s| s.parse::<T>()).collect()
}

pub fn pretty_print(map: &Vec<String>) {
    for row in map {
        for ch in row.chars() {
            print!("{ } ", ch)
        }
        println!()
    }
}
