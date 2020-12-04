use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

fn find_n_numbers(input: &Vec<usize>, n: usize) -> usize {
    assert!(n > 0);
    let input_index: Vec<(usize, usize)> = input.iter().copied().enumerate().collect();
    let mut numbers: Vec<(usize, usize)> = Vec::with_capacity(n);

    let mut index = 0usize;
    loop {
        for (i, number) in input_index[index..].iter() {
            let sum: usize = *number + numbers.iter().map(|(_, x)| x).sum::<usize>();
            if numbers.len() == n - 1 {
                //last number only accept if == 2020
                if sum == 2020 {
                    numbers.push((*i, *number));
                    break;
                }
            } else {
                if sum > 2020 {
                    continue; //too big, no other value could result in 2020
                } else {
                    numbers.push((*i, *number));
                }
            }
        }
        if numbers.len() == n {
            break; //found the correct sum
        } else {
            match numbers.pop() {
                None => panic!("Unable to find the number!"),
                Some((i, _)) => index = i + 1,
            }
        }
    }
    let mut ret = 1;
    for (_, x) in numbers {
        ret *= x;
    }
    ret
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read_to_string(file_path).expect("Unable to read the File");
    let input: Vec<usize> = file
        .lines()
        .map(|x| x.parse::<usize>().expect("Invalid input"))
        .collect();
    let two_numbers = find_n_numbers(&input, 2);
    println!("Found Two numbers {}", two_numbers);
    let three_numbers = find_n_numbers(&input, 3);
    println!("Found Three numbers {}", three_numbers);
}
