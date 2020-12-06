use std::collections::HashMap;
use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read(file_path).expect("Unable to read the File");

    //will split the file into by '\n\n', so we get a iter for each group
    let mut new_group_flag = false;
    let groups = file.split(|&x| {
        if new_group_flag {
            new_group_flag = false;
            if x == b'\n' {
                return true;
            }
        } else {
            if x == b'\n' {
                new_group_flag = true;
            }
        }
        false
    });

    let mut p1_res = 0usize;
    let mut p2_res = 0usize;
    for group in groups {
        let mut num_person = 0usize;
        let mut group_answers: HashMap<u8, usize> = HashMap::new();
        for person in group.split(|&x| x == b'\n') {
            //bug: this happen because the last line end with \n
            if person.len() == 0 {
                continue;
            }
            num_person += 1;
            for &answer in person {
                let entry = group_answers.entry(answer).or_insert(0);
                *entry += 1;
            }
        }
        p1_res += group_answers.len();
        for v in group_answers.values() {
            if *v == num_person {
                p2_res += 1;
            }
        }
    }
    println!("P1: {}", p1_res);
    println!("P2: {}", p2_res);
}
