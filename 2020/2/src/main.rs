use regex::Regex;
use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

fn main() {
    // assuming all passwords are lower case
    let regex = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read_to_string(file_path).expect("Unable to read the File");
    let mut valid_count = 0;
    for cap in regex.captures_iter(file.as_str()) {
        let (min, max, chr, pass) = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<char>().unwrap(),
            cap[4].to_string(),
        );
        assert!(min <= max);
        let mut chr_count = 0;
        for i in pass.chars() {
            if i == chr {
                chr_count += 1;
            }
        }
        if chr_count <= max && chr_count >= min {
            valid_count += 1;
        }
    }
    println!("valid passwords {}", valid_count);
}
