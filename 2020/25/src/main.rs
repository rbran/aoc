use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

struct Input {
    door_pk: usize,
    card_pk: usize,
}

fn error(s: &str) -> Err {
    Box::new(Error::new(InvalidData, s.to_string()))
}

impl FromStr for Input {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|x| x.parse::<usize>())
            .take(2)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Input {
            door_pk: lines[1],
            card_pk: lines[0],
        })
    }
}

struct Part1<'a> {
    input: &'a Input,
    door_loop: usize,
    card_loop: usize,
}

impl<'a> TryFrom<&'a Input> for Part1<'a> {
    type Error = Err;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        Ok(Part1 {
            input,
            door_loop: 0,
            card_loop: 0,
        })
    }
}

impl<'a> Part1<'a> {
    fn solve(&mut self) -> Result<usize, Err> {
        //the moduly to the end makes me think I can solve this with chinese
        //reminder teory, bot for now I'll brute
        let brute = |x| {
            let mut value = 1;
            let mut loops = 0;
            while value != x {
                value = (value * 7) % 20201227;
                loops += 1;
            }
            loops
        };
        let calc_key = |loops, pk| {
            (0..loops)
                .into_iter()
                .fold(1, |acc, _| (acc * pk) % 20201227)
        };

        //brute loops for door and card
        self.door_loop = brute(self.input.door_pk);
        self.card_loop = brute(self.input.card_pk);

        let door_key = calc_key(self.door_loop, self.input.card_pk);
        let card_key = calc_key(self.card_loop, self.input.door_pk);

        if door_key != card_key {
            Err(error("Invalid PK"))
        } else {
            Ok(door_key)
        }
    }
}

fn main() -> Result<(), Err> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    println!("P1: {}", part1.solve()?);
    Ok(())
}

#[test]
fn test_example() -> Result<(), Err> {
    const INPUT: &str = "\
5764801
17807724";
    let input: Input = INPUT.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    assert_eq!(part1.solve()?, 14897079);
    Ok(())
}
