use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read_to_string(file_path).expect("Unable to read the File");

    let mut p1_res = 0u16;
    for line in file.lines() {
        let mut id = 0u16;
        for (i, b) in line.chars().rev().enumerate() {
            let b = match b {
                'R' | 'r' | 'B' | 'b' => 1u16,
                'L' | 'l' | 'F' | 'f' => 0u16,
                _ => panic!("Unknown input {}", b),
            };

            id |= b << (i as u16);
            if id > p1_res {
                p1_res = id;
            }
        }
    }
    println!("P1: {}", p1_res);
}
