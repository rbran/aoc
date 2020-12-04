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
    let mut p1_valid_count = 0; //part 1 counter
    let mut p2_valid_count = 0; //part 2 counter
    for cap in regex.captures_iter(file.as_str()) {
        let (min, max, chr, pass) = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<char>().unwrap(),
            cap[4].to_string(),
        );
        assert!(min <= max);
        // check part 1
        let mut chr_count = 0;
        for i in pass.chars() {
            if i == chr {
                chr_count += 1;
            }
        }
        if chr_count <= max && chr_count >= min {
            p1_valid_count += 1;
        }
        //check part 2
        let mut pass_iter = pass.chars();
        let first = pass_iter.nth(min - 1).unwrap();
        let second = pass_iter.nth(max - min - 1).unwrap(); //reuse the same iterator
        p2_valid_count += if first == second { 0 }
        else if first == chr || second == chr { 1 }
        else { 0 };
    }
    println!("p1 {}\np2 {}", p1_valid_count, p2_valid_count);
}
