use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn is_valid(input: &Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
    if input.len() == 0 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            "Input is empty",
        )));
    }

    if input[0] == 0 || input[0] > 3 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            "Smallest input is invalid, should be 1, 2, 3",
        )));
    }

    if input
        .windows(2)
        .find(|x| match x[1] - x[0] {
            1 | 2 | 3 => false,
            _ => true,
        })
        .is_some()
    {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            "Unable to use all adapters",
        )));
    }

    Ok(())
}

fn solve1(input: &Vec<usize>) -> usize {
    //your device's built-in adapter is always 3 higher than the highest
    //adapter, so its rating is 22 jolts (always a difference of 3).
    //so diff 3 start with one

    //contain the diff jolts between adapters
    let mut diff = [0, 0, 0, 1];
    //account the connection from the outlet (0 jolts) to the first adapter
    diff[input[0]] += 1;
    //count the diference in jolts between adapters
    input.windows(2).for_each(|x| diff[x[1] - x[0]] += 1);
    diff[1] * diff[3]
}

struct Tribonacci {
    numbers: Vec<usize>,
}

impl Tribonacci {
    fn new() -> Self {
        Tribonacci {
            numbers: vec![1, 1, 2, 4, 7, 13, 24],
        }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.numbers.len() > x {
            return self.numbers[x];
        }
        let start = self.numbers.len();
        for i in start..=x {
            self.numbers
                .push(self.numbers[i - 1] + self.numbers[i - 2] + self.numbers[i - 3]);
        }
        self.numbers[x]
    }
}

fn solve2(input: &Vec<usize>) -> usize {
    //I'll not explay why I need Tribonacci
    let mut tri = Tribonacci::new();
    let mut acc = input[0]; //start with diff from outlet to the first adapter
    input.windows(2).filter_map(|x| {
        if x[1] - x[0] == 1 {
            acc += 1;
            None
        } else { // = 3
            if acc < 2 {
                acc = 0;
                None
            } else {
                let calc = acc;
                acc = 0;
                Some(tri.get(calc))
            }
        }
    }).product::<usize>() * tri.get(acc) // the last is not processed
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut input = input
        .lines()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    input.sort();
    is_valid(&mut input)?;
    println!("P1: {}", solve1(&input));
    println!("P2: {}", solve2(&input));
    Ok(())
}

const _TEST_INPUTS: [&[usize]; 2] = [
    &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4],
    &[
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ],
];

#[test]
fn test_part1_input0() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = _TEST_INPUTS[0].to_vec();
    input.sort();
    is_valid(&input)?;
    assert_eq!(solve1(&input), 7 * 5);
    Ok(())
}

#[test]
fn test_part1_input1() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = _TEST_INPUTS[1].to_vec();
    input.sort();
    is_valid(&input)?;
    assert_eq!(solve1(&input), 22 * 10);
    Ok(())
}

#[test]
fn test_part2_input0() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = _TEST_INPUTS[0].to_vec();
    input.sort();
    is_valid(&input)?;
    assert_eq!(solve2(&input), 8);
    Ok(())
}

#[test]
fn test_part2_input1() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = _TEST_INPUTS[1].to_vec();
    input.sort();
    is_valid(&input)?;
    assert_eq!(solve2(&input), 19208);
    Ok(())
}
