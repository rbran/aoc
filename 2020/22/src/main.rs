use std::collections::HashSet;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

struct Input {
    p1: Vec<usize>,
    p2: Vec<usize>,
}

fn error(s: &str) -> Err {
    Box::new(Error::new(InvalidData, s.to_string()))
}

impl FromStr for Input {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let get_dick =
            |lines: &mut std::str::Lines| -> Result<Vec<usize>, Self::Err> {
                let mut p = Vec::new();
                loop {
                    if let Some(line) = lines.next() {
                        if line.len() == 0 {
                            break;
                        }
                        p.push(line.parse()?);
                    } else {
                        break;
                    }
                }
                Ok(p)
            };

        match lines.next() {
            Some("Player 1:") => (),
            _ => return Err(error("")),
        }
        let p1 = get_dick(&mut lines)?;
        match lines.next() {
            Some("Player 2:") => (),
            _ => return Err(error("")),
        }
        let p2 = get_dick(&mut lines)?;
        Ok(Input { p1, p2 })
    }
}

struct Part1<'a> {
    input: &'a Input,
    winner: Vec<usize>,
}

impl<'a> TryFrom<&'a Input> for Part1<'a> {
    type Error = Err;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        Ok(Part1 {
            input,
            winner: Vec::new(),
        })
    }
}

impl<'a> Part1<'a> {
    fn solve(&mut self) -> Result<usize, Err> {
        let max_size = self.input.p1.len() + self.input.p2.len();
        let mut p1: Vec<usize> = Vec::with_capacity(max_size);
        let mut p2: Vec<usize> = Vec::with_capacity(max_size);
        p1.extend(self.input.p1.iter());
        p2.extend(self.input.p2.iter());
        self.winner = loop {
            if p1.len() == 0 {
                break p2;
            }
            if p2.len() == 0 {
                break p1;
            }

            let p1d = p1.remove(0);
            let p2d = p2.remove(0);
            if p1d < p2d {
                p2.push(p2d);
                p2.push(p1d);
            } else {
                p1.push(p1d);
                p1.push(p2d);
            }
        };
        Ok(self
            .winner
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, v)| acc + (v * (i + 1))))
    }
}

struct Part2<'a> {
    input: &'a Input,
}

impl<'a> TryFrom<&'a Part1<'a>> for Part2<'a> {
    type Error = Err;
    fn try_from(part1: &'a Part1) -> Result<Self, Self::Error> {
        Ok(Part2 { input: part1.input })
    }
}

impl<'a> Part2<'a> {
    fn solve(&mut self) -> Result<usize, Err> {
        let (_, ret) = Part2::get_winner_rec(&self.input.p1, &self.input.p2);
        Ok(ret)
    }

    fn get_winner_rec(p1: &[usize], p2: &[usize]) -> (bool, usize) {
        let calc_points = |winner: Vec<usize>| -> usize {
            winner
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, v)| acc + (v * (i + 1)))
        };

        let mut p1 = p1.to_vec();
        let mut p2 = p2.to_vec();
        let mut rounds: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
        loop {
            match (p1.len(), p2.len()) {
                (0, 0) => panic!("Nope"),
                (_, 0) => return (true, calc_points(p1)),
                (0, _) => return (false, calc_points(p2)),
                _ => (),
            }
            //if this round happend before, p1 win
            if rounds.contains(&(p1.clone(), p2.clone())) {
                return (true, calc_points(p1));
            }
            //add this round to the history
            rounds.insert((p1.clone(), p2.clone()));
            let (card_p1, card_p2) = (p1.remove(0), p2.remove(0));
            //recursive game
            let p1_win_round = if card_p1 <= p1.len() && card_p2 <= p2.len() {
                //try to avoid recursion
                let max_p1 = *p1.iter().take(card_p1).max().unwrap();
                let max_p2 = *p2.iter().take(card_p2).max().unwrap();
                //the biggest number can never leave the player hand, so this
                //player can never lose, unless it loops, in this case, p1 win
                if max_p1 > max_p2 {
                    true
                } else {
                    let (ret, _) =
                        Part2::get_winner_rec(&p1[0..card_p1], &p2[0..card_p2]);
                    ret
                }
            } else {
                card_p1 > card_p2
            };

            if p1_win_round {
                p1.push(card_p1);
                p1.push(card_p2);
            } else {
                p2.push(card_p2);
                p2.push(card_p1);
            }
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
    let mut part2 = Part2::try_from(&part1)?;
    println!("P2: {}", part2.solve()?);
    Ok(())
}

#[test]
fn test_example() -> Result<(), Err> {
    const INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    let input: Input = INPUT.parse()?;
    assert_eq!(input.p1, vec![9, 2, 6, 3, 1]);
    assert_eq!(input.p2, vec![5, 8, 4, 7, 10]);
    let mut part1 = Part1::try_from(&input)?;
    assert_eq!(part1.solve()?, 306);
    let mut part2 = Part2::try_from(&part1)?;
    assert_eq!(part2.solve()?, 291);
    Ok(())
}
