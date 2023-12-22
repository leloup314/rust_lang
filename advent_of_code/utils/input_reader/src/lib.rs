use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn load_ints_from_file(file_path: &str) -> Vec<i64> {
    // Open file
    let f = File::open(file_path).expect("File not found!");
    // Prepare buffer
    let reader = BufReader::new(f);
    // Loop over buffer and fill vector
    let ints: Vec<i64> = reader.lines()
                               .map(|line| line.unwrap()
                                               .parse::<i64>()
                                               .unwrap())
                               .collect();
    ints
}


pub fn load_lines_from_file(file_path: &str) -> Vec<String> {
    // Open file
    let f = File::open(file_path).expect("File not found!");
    // Prepare buffer
    let reader = BufReader::new(f);
    // Loop over buffer and fill vector
    let strs: Vec<String> = reader.lines()
                                  .map(|line| line.unwrap())
                                  .collect();
    strs
}
