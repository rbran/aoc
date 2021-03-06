use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

#[derive(Debug)]
struct Expression {
    ele: Vec<(Element, Operation)>,
}

#[derive(Debug)]
enum Operation {
    Sum,
    Mut,
    Equal, //mark the end of expression
}

#[derive(Debug)]
enum Element {
    Num(usize),
    Exp(Expression),
}

#[derive(Debug)]
struct Input {
    exp: Vec<Expression>,
}

impl Expression {
    fn from_chars(
        chars: &mut impl Iterator<Item = char>,
    ) -> Result<Self, Box<Error>> {
        let mut ele = Vec::new();
        loop {
            let new_ele = match chars.next() {
                Some(x @ '0'..='9') => {
                    Element::Num(x.to_digit(10).ok_or_else(|| {
                        Box::new(Error::new(
                            InvalidData,
                            "Unable to parse element",
                        ))
                    })? as usize)
                }
                Some('(') => Element::Exp(Expression::from_chars(chars)?),
                //TODO check if we are in parenteses
                None => break,
                Some(_) => {
                    return Err(Box::new(Error::new(
                        InvalidData,
                        "Invalid input element",
                    )))
                }
            };
            let new_op = match chars.next() {
                Some('+') => Operation::Sum,
                Some('*') => Operation::Mut,
                //TODO check if we are in parenteses
                Some(')') | None => {
                    ele.push((new_ele, Operation::Equal));
                    break;
                }
                Some(_) => {
                    return Err(Box::new(Error::new(
                        InvalidData,
                        "Invalid input operation",
                    )))
                }
            };
            ele.push((new_ele, new_op));
        }
        if ele.len() == 0 {
            return Err(Box::new(Error::new(InvalidData, "Empty expression")));
        }
        match ele.last().unwrap() {
            (_, Operation::Equal) => (),
            _ => {
                return Err(Box::new(Error::new(
                    InvalidData,
                    "Expression end incorrectly",
                )))
            }
        }
        Ok(Expression { ele })
    }
    //TODO: convert to RPN
    fn solve(&self) -> usize {
        let mut list = self.ele.iter();
        let (first_ele, op) = list.next().unwrap();
        let mut res = match first_ele {
            Element::Num(x) => *x,
            Element::Exp(x) => x.solve(),
        };
        let mut op = op;
        for (ele, next_op) in list {
            let ele = match ele {
                Element::Num(x) => *x,
                Element::Exp(x) => x.solve(),
            };
            match op {
                Operation::Sum => res += ele,
                Operation::Mut => res *= ele,
                Operation::Equal => panic!("should never get here"),
            }
            op = next_op;
        }
        res
    }

    fn solve2(&self) -> usize {
        //first pass, evalutate everything
        let stack1 = self.ele.iter().map(|(ele, op)| {
            let ele = match ele {
                Element::Num(x) => *x,
                Element::Exp(x) => x.solve2(),
            };
            (ele, op)
        }).collect::<Vec<_>>();

        //second pass, solve sums
        let mut rolling = false;
        let mut last_res = 0;
        let stack2 = stack1.windows(2).filter_map(|x| {
            let (n1, op) = x[0];
            let (n2, _) = x[1];
            match op {
                Operation::Sum => {
                    if rolling {
                        last_res += n2;
                        None
                    } else {
                        rolling = true;
                        last_res = n1 + n2;
                        None
                    }
                },
                _ => {
                    if rolling {
                        rolling = false;
                        Some(last_res)
                    } else {
                        Some(n1)
                    }
                }
            }
        }).collect::<Vec<_>>();
        //the last element we need to add manually
        let fill: [usize; 1] = if !rolling {
            [stack1.last().unwrap().0]
        } else {
            [last_res]
        };

        //final pass, multiply it all
        stack2.iter().chain(&fill).product()
    }

}

impl FromStr for Input {
    type Err = Box<Error>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            exp: input
                .lines()
                .map(|x| {
                    Expression::from_chars(
                        &mut x.chars().filter(|x| !x.is_whitespace()),
                    )
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

fn solve1(input: &Input) -> Result<usize, Box<Error>> {
    Ok(input.exp.iter().map(|x| x.solve()).sum())
}

fn solve2(input: &Input) -> Result<usize, Box<Error>> {
    Ok(input.exp.iter().map(|x| x.solve2()).sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    println!("P1: {}", solve1(&input)?);
    println!("P2: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Box<Error>> {
    const INPUTS: &[(&str, usize)] = &[
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];
    for (input, res) in INPUTS.iter() {
        let input = input.parse()?;
        assert_eq!(solve1(&input)?, *res);
    }
    Ok(())
}

#[test]
fn test_part2() -> Result<(), Box<Error>> {
    const INPUTS: &[(&str, usize)] = &[
        ("1 + 2 * 3 + 4 * 5 + 6", 231),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    ];
    for (input, res) in INPUTS.iter() {
        let input = input.parse()?;
        assert_eq!(solve2(&input)?, *res);
    }
    Ok(())
}
