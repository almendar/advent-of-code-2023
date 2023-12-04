use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

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

pub fn pretty_print(map: &Vec<String>) {
    for row in map {
        for ch in row.chars() {
            print!("{ } ", ch)
        }
        println!()
    }
}
