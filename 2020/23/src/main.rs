use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

struct Input {
    cups: Vec<u32>,
}

fn error(s: &str) -> Err {
    Box::new(Error::new(InvalidData, s.to_string()))
}

impl FromStr for Input {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups = s
            .chars()
            .filter_map(|x| match x {
                '0'..='9' => Some(Ok(x.to_digit(10).unwrap())),
                '\n' | '\r' | '\x00' => None,
                _ => Some(Err(error(""))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        //verify if valid
        //expected a sequence of number 0..x of unique numbers in a random order
        let mut check = cups.to_vec();
        check.sort();
        if check.windows(2).find(|x| x[0] == x[1]).is_some() {
            return Err(error("repeated input"));
        }
        if (check[check.len() - 1] - check[0]) as usize != check.len() - 1 {
            return Err(error("invalid range"));
        }
        Ok(Input { cups })
    }
}

struct Part1<'a> {
    input: &'a Input,
    result: Vec<u32>,
}

impl<'a> TryFrom<&'a Input> for Part1<'a> {
    type Error = Err;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        let result = Vec::with_capacity(input.cups.len() - 1);
        Ok(Part1 { input, result })
    }
}

impl<'a> Part1<'a> {
    fn solve(&mut self, rounds: usize) -> Result<String, Err> {
        self.result.clear();
        //instead of using ring buffer, I'll rotate between this two vectors
        let (mut a, mut b) = (
            &mut self.input.cups.to_vec(),
            &mut Vec::with_capacity(self.input.cups.len()),
        );
        //Each move, the crab does the following actions:
        for _ in 0..rounds + 1 {
            //Before the crab starts, it will designate the first cup in your
            //list as the current cup.
            let cup = a[0];
            //The crab picks up the three cups that are immediately clockwise
            //of the current cup. They are removed from the circle; cup spacing
            //is adjusted as necessary to maintain the circle.
            let pick_up = [a[1], a[2], a[3]];
            //The crab selects a destination cup: the cup with a label equal to
            //the current cup's label minus one
            let mut dest = cup;
            //If this would select one of the cups that was just picked up,
            //the crab will keep subtracting one until it finds a cup that
            //wasn't just picked up.
            while pick_up.contains(&(dest as u32)) || dest == cup {
                if dest != 1 {
                    dest -= 1;
                } else {
                    //If at any point in this process the value goes below the
                    //lowest value on any cup's label, it wraps around to the
                    //highest value on any cup's label instead.
                    dest = a.len() as u32;
                }
            }
            let dest_index = a.iter().position(|&x| x == dest).unwrap();

            b.clear();
            if dest_index + 1 > 4 {
                b.extend(a[4..dest_index + 1].iter());
            }
            b.extend(pick_up.iter());
            b.extend(a[dest_index + 1..].iter());
            b.push(cup);

            //rotate vectors
            let tmp = b;
            b = a;
            a = tmp;
        }

        //After the crab is done, what order will the cups be in? Starting
        //after the cup labeled 1, collect the other cups' labels clockwise
        //into a single string with no extra characters; each number except
        //1 should appear exactly once.
        let start_index = b.iter().position(|&x| x == 1).unwrap();
        self.result.extend(b[start_index + 1..].iter());
        self.result.extend(b[..start_index].iter());

        Ok(self
            .result
            .iter()
            .map(|&x| std::char::from_digit(x, 10).unwrap())
            .collect())
    }
}

struct Part2<'a> {
    input: &'a Input,
}

impl<'a> TryFrom<&'a Part1<'a>> for Part2<'a> {
    type Error = Err;
    fn try_from(part1: &'a Part1) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl<'a> Part2<'a> {
    fn solve(&mut self) -> Result<String, Err> {
        unimplemented!()
    }
}

fn main() -> Result<(), Err> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    println!("P1: {}", part1.solve(100)?);
    let mut part2 = Part2::try_from(&part1)?;
    println!("P2: {}", part2.solve()?);
    Ok(())
}

#[test]
fn test_example() -> Result<(), Err> {
    const INPUT: &str = "389125467";
    let input: Input = INPUT.parse()?;
    assert_eq!(input.cups, vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    let mut part1 = Part1::try_from(&input)?;
    assert_eq!(part1.solve(10)?, "92658374");
    assert_eq!(part1.solve(100)?, "67384529");
    let mut part2 = Part2::try_from(&part1)?;
    assert_eq!(part2.solve()?, "");
    Ok(())
}
