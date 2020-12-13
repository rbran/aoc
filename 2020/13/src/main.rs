use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

struct Service {
    lines: Vec<Option<usize>>,
    time: usize,
}

impl FromStr for Service {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time = lines
            .next()
            .ok_or_else(|| Error::new(InvalidData, "Unable to find the depart time"))?
            .parse::<usize>()
            .or_else(|_| Err(Error::new(InvalidData, "Unable to parse the depart time")))?;
        let lines = lines
            .next()
            .ok_or_else(|| Error::new(InvalidData, "Unable to find the bus lines"))?;

        let lines = lines
            .split(',')
            .map(|x| {
                if x.len() == 0 {
                    return Err(Error::new(InvalidData, "Unable to parse empty bus id"));
                }
                match x {
                    "x" => Ok(None),
                    _ => Ok(Some(x.parse::<usize>().or_else(|_| {
                        Err(Error::new(InvalidData, "Unable to parse the bus id"))
                    })?)),
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(Service { lines, time })
    }
}

fn solve1(input: &str) -> Result<usize, Box<Error>> {
    let service = input.parse::<Service>()?;
    let time = service.time;
    let mut smallest: Option<(usize, usize)> = None;
    for id in service.lines.iter().filter_map(|&x| x) {
        let wait = id - (time % id);
        match smallest {
            None => smallest = Some((id, wait)),
            Some((_, small_wait)) => {
                if small_wait > wait {
                    smallest = Some((id, wait));
                }
            }
        }
    }
    let smallest =
        smallest.ok_or_else(|| Error::new(InvalidData, "Unable to find any bus line"))?;
    Ok(smallest.0 * smallest.1)
}

fn solve2(input: &str) -> Result<usize, Box<Error>> {
    // Chinese remainder stuff
    let mut global_coprime: usize = 1;
    let buses: Vec<(usize, usize)> = input
        .parse::<Service>()?
        .lines
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            None => None,
            Some(x) => {
                global_coprime *= *x;
                Some((i, *x))
            }
        })
        .collect();

    let mut factors = Vec::with_capacity(buses.len());

    for (i, bus) in buses.iter() {
        let factor: usize = (bus - (i % bus)) % bus;
        let current_factor: usize = global_coprime / bus;
        let mut acc_factor = current_factor;
        while acc_factor % bus != factor {
            acc_factor += current_factor;
        }
        factors.push(acc_factor);
    }

    Ok(factors.iter().sum::<usize>() % global_coprime)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    println!("P1: {}", solve1(&input)?);
    println!("P1: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n7,13,x,x,59,x,31,19";
    assert_eq!(solve1(INPUT)?, 295);
    Ok(())
}

#[test]
fn test_part_2_1() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n7,13,x,x,59,x,31,19";
    assert_eq!(solve2(INPUT)?, 1068781);
    Ok(())
}

#[test]
fn test_part_2_2() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n17,x,13,19";
    assert_eq!(solve2(INPUT)?, 3417);
    Ok(())
}

#[test]
fn test_part_2_3() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n67,7,59,61";
    assert_eq!(solve2(INPUT)?, 754018);
    Ok(())
}

#[test]
fn test_part_2_4() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n67,x,7,59,61";
    assert_eq!(solve2(INPUT)?, 779210);
    Ok(())
}

#[test]
fn test_part_2_5() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n67,7,x,59,61";
    assert_eq!(solve2(INPUT)?, 1261476);
    Ok(())
}

#[test]
fn test_part_2_6() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "939\n1789,37,47,1889";
    assert_eq!(solve2(INPUT)?, 1202161486);
    Ok(())
}
