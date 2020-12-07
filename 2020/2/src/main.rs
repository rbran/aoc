use regex::Regex;
use std::env;
use std::fs;

fn main() -> Result <(), Box<dyn std::error::Error>> {
    // assuming all passwords are lower case
    let regex = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut p1_valid_count = 0; //part 1 counter
    let mut p2_valid_count = 0; //part 2 counter
    for cap in regex.captures_iter(&input) {
        let (min, max, chr, pass) = (
            cap[1].parse::<usize>()?,
            cap[2].parse::<usize>()?,
            cap[3].parse::<char>()?,
            cap[4].to_string(),
        );
        assert!(min <= max);
        // check part 1
        let chr_count = pass.chars().filter(|&x| x == chr).count();
        if chr_count <= max && chr_count >= min {
            p1_valid_count += 1;
        }
        //check part 2
        let mut pass_iter = pass.chars();
        let first = pass_iter.nth(min - 1);
        let second = pass_iter.nth(max - min - 1); //reuse the same iterator

        if (first == Some(chr)) ^ (second == Some(chr)) {
            p2_valid_count += 1
        }
    }
    println!("p1 {}\np2 {}", p1_valid_count, p2_valid_count);
    Ok(())
}
