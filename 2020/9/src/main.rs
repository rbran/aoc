use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn find_invalid(input: &[usize]) -> Option<(usize, usize)> {
    for (index, &value) in (&input[25..]).iter().enumerate() {
        let input_slice = &input[index..index + 25];
        let found = input_slice.iter().find(|&&x| {
            let other_value = if x > value { x - value } else { value - x };
            input_slice.contains(&other_value)
        });
        if found.is_none() {
            return Some((value, index));
        }
    }
    None
}

fn find_sum(input: &[usize], number: usize) -> Option<usize> {
    (0..(input.len())).find_map(|start| {
        //search all the input indexes
        //check if we can find a contiguous list that a sum is eq number
        input[start..]
            .iter()
            .scan(0, |sum, &v| {
                //sum all the values
                *sum += v;
                Some(*sum)
            })
            .enumerate()
            .find(|&(_, v)| v == number)
            .and_then(|(i, _)| {
                //if found, return the sum of the smallest and biggest values
                let mut vec = input[start..=start + i].to_vec();
                vec.sort();
                return Some(vec.first()? + vec.last()?);
            })
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // assuming all passwords are lower case
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut input = input
        .lines()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let (p1_res, p1_index) = find_invalid(&input).ok_or(Error::new(
        ErrorKind::InvalidData,
        "Unable to find Solution 1 on file",
    ))?;
    println!("P1: {}", p1_res);
    input.remove(p1_index);
    let p2_res = find_sum(&input, p1_res).ok_or(Error::new(
        ErrorKind::InvalidData,
        "Unable to find Solution 2 on file",
    ))?;
    println!("P2: {}", p2_res);
    Ok(())
}
