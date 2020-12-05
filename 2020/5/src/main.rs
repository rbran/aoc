use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read_to_string(file_path).expect("Unable to read the File");

    //At first, I didn't understood exacly what the AoC meant by
    //"some of the seats at the very front and back of the plane"
    //so I did this, but was too lazy to improve it, maybe in the future
    let mut dodge_this = Vec::with_capacity(1024);
    for i in 0u16..1024 {
        dodge_this.push(i);
    }
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
        }
        if id > p1_res {
            p1_res = id;
        }
        dodge_this.remove(dodge_this.binary_search(&id).unwrap());
    }
    println!("P1: {}", p1_res);
    //println!("Smith: {:?}", dodge_this);
    //we should get a vector with the format [0,1,2,...,56,456,667,668...]
    //and 456 is our number, so just check when the index is diferent
    //from the value, the first time that happen, we got out number
    let (_, p2_res) = dodge_this
        .iter()
        .enumerate()
        .find(|(i, &v)| *i != v as usize)
        .unwrap();
    println!("P2: {}", p2_res);
}
