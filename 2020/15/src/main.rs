use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

fn parse(input: &str) -> Result<Vec<usize>, Box<Error>> {
    // TODO: check if numbers are unique and there is one or more
    Ok(input
        .trim()
        .split(',')
        .map(|x| {
            Ok(x.parse::<usize>()
                .or_else(|_| Err(Error::new(InvalidData, "Unable to parse input")))?)
        })
        .collect::<Result<_, Error>>()?)
}

fn solve1(input: &str) -> Result<usize, Box<Error>> {
    let mut game = parse(input)?;
    let mut last_number = game.last().copied().unwrap();
    for _ in game.len()..2020 {
        let spoken: Vec<(usize, &usize)> = game
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &x)| x == last_number)
            .take(2)
            .collect();
        if spoken.len() != 2 {
            last_number = 0;
        } else {
            last_number = spoken[0].0 - spoken[1].0;
        }
        game.push(last_number);
    }
    Ok(game.last().copied().unwrap())
}

fn solve2(input: &str) -> Result<usize, Box<Error>> {
    // same as solve 1, but optimized to reduze memory use
    // I should think better about this, probably can optimize it greatly.
    // although this works :)
    let mut game: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();
    let input = parse(input)?;
    for (i, v) in input.iter().enumerate() {
        game.insert(*v, (Some(i), None));
    }
    let mut last_number = input.last().copied().unwrap();
    for turn in input.len()..30000000 {
        // calculate the number to speak
        let indexes = game.get(&last_number).unwrap();
        let next_number = match indexes {
            // the last number happend twice or more
            (Some(x), Some(y)) => *x - *y,
            // the last number only happend once
            _ => 0,
        };
        // insert the next number to the game (speak it)
        let entry = game.entry(next_number).or_insert((None, None));
        entry.1 = entry.0;
        entry.0 = Some(turn);
        last_number = next_number;
    }
    Ok(last_number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    println!("P1: {}", solve1(&input)?);
    println!("P2: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    const INPUTS: &[(&str, usize)] = &[
        ("0,3,6", 436),
        ("1,3,2", 1),
        ("2,1,3", 10),
        ("1,2,3", 27),
        ("2,3,1", 78),
        ("3,2,1", 438),
        ("3,1,2", 1836),
    ];
    for (input, result) in INPUTS {
        assert_eq!(solve1(input)?, *result);
    }
    Ok(())
}

#[test]
fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
    const INPUTS: &[(&str, usize)] = &[
        ("0,3,6", 175594),
        ("1,3,2", 2578),
        ("2,1,3", 3544142),
        ("1,2,3", 261214),
        ("2,3,1", 6895259),
        ("3,2,1", 18),
        ("3,1,2", 362),
    ];
    for (input, result) in INPUTS {
        assert_eq!(solve2(input)?, *result);
    }
    Ok(())
}
