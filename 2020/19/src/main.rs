//the solution 1 is terrible and slow

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rule {
    Value(char),
    Alias(usize),
    Dual(usize, usize),
    Or(usize, usize),
    OrDual(usize, usize, usize, usize),
}

#[derive(Debug, PartialEq, Clone)]
struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl FromStr for Rule {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEXS: [Regex; 5] = [
                Regex::new("^\"([a-z])\"$").unwrap(),
                Regex::new("^(\\d+)$").unwrap(),
                Regex::new("^(\\d+)\\s+(\\d+)$").unwrap(),
                Regex::new("^(\\d+)\\s*\\|\\s*(\\d+)$").unwrap(),
                Regex::new("^(\\d+)\\s+(\\d+)\\s*\\|\\s*(\\d+)\\s+(\\d+)$")
                    .unwrap(),
            ];
        }

        let error = |x| Box::new(Error::new(InvalidData, x));
        for (index, re) in REGEXS.iter().enumerate() {
            if let Some(x) = re.captures(s) {
                let ret = match index {
                    0 => Ok(Rule::Value(x[1].chars().next().unwrap())),
                    1 => Ok(Rule::Alias(x[1].parse().or_else(|_| {
                        Err(error("Unable to parse \"Alias\""))
                    })?)),
                    2 => Ok(Rule::Dual(
                        x[1].parse().or_else(|_| {
                            Err(error("Unable to parse \"Dual\" 1"))
                        })?,
                        x[2].parse().or_else(|_| {
                            Err(error("Unable to parse \"Dual\" 2"))
                        })?,
                    )),
                    3 => Ok(Rule::Or(
                        x[1].parse().or_else(|_| {
                            Err(error("Unable to parse \"Or\" 1"))
                        })?,
                        x[2].parse().or_else(|_| {
                            Err(error("Unable to parse \"Or\" 2"))
                        })?,
                    )),
                    4 => Ok(Rule::OrDual(
                        x[1].parse().or_else(|_| {
                            Err(error("Unable to parse \"DualOr\" 1"))
                        })?,
                        x[2].parse().or_else(|_| {
                            Err(error("Unable to parse \"DualOr\" 2"))
                        })?,
                        x[3].parse().or_else(|_| {
                            Err(error("Unable to parse \"DualOr\" 3"))
                        })?,
                        x[4].parse().or_else(|_| {
                            Err(error("Unable to parse \"DualOr\" 4"))
                        })?,
                    )),
                    _ => panic!("Regex not implemented"),
                };
                return ret;
            }
        }
        return Err(error("Unable to match value"));
    }
}

impl FromStr for Input {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = |x| Box::new(Error::new(InvalidData, x));

        let mut input = s.split("\n\n");
        let input_rules = input.next().ok_or_else(|| error("no rules"))?;
        let input_values = input.next().ok_or_else(|| error("no values"))?;

        //parse rules
        let mut rules = HashMap::new();
        for rule_line in input_rules.lines() {
            let mut rule_line = rule_line.split(':');
            let rule_index =
                rule_line.next().ok_or_else(|| error("no index"))?;
            let rule_index = rule_index
                .parse::<usize>()
                .or_else(|_| Err(error("index not parsable")))?;
            let rule_value =
                rule_line.next().ok_or_else(|| error("no value"))?;
            let rule_value = rule_value.trim().parse::<Rule>()?;
            //TODO check index is duplicated
            rules.insert(rule_index, rule_value);
        }
        //TODO check number of rules

        //parse value
        let messages = input_values.lines().map(|x| x.to_string()).collect();
        Ok(Input { rules, messages })
    }
}

fn combine_eval(x: &Vec<String>, y: &Vec<String>) -> Vec<String> {
    let mut new = Vec::new();
    for value1 in x {
        for value2 in y {
            new.push(value1.to_owned() + value2);
        }
    }
    new
}

fn fork_eval(x: &Vec<String>, y: &Vec<String>) -> Vec<String> {
    x.iter().cloned().chain(y.iter().cloned()).collect()
}

fn solve1(input: &Input) -> Result<usize, Box<Error>> {
    //I'll evaluate the rule 0 to all possible results.
    //I should evaluate only the necessary rules, but instead I'll evaluate all
    let mut eval: HashMap<usize, Vec<String>> = HashMap::new();
    let number_rules = input.rules.len();
    loop {
        for (index, rule) in input.rules.iter() {
            println!("{}", *index);
            //if already eval, skip
            if eval.contains_key(index) {
                continue;
            }
            match rule {
                Rule::Value(x) => {
                    //just insert into eval, if didn't exist already
                    eval.insert(*index, vec![x.to_string()]);
                }
                Rule::Alias(x) => {
                    //check if the alias is already eval, if not skip
                    if eval.contains_key(x) {
                        eval.insert(*index, eval.get(x).unwrap().to_owned());
                    }
                }
                Rule::Dual(x, y) => {
                    //check if both are eval, if so, combine both
                    if eval.contains_key(x) && eval.contains_key(y) {
                        eval.insert(
                            *index,
                            combine_eval(
                                eval.get(x).unwrap(),
                                eval.get(y).unwrap(),
                            ),
                        );
                    }
                }
                Rule::Or(x, y) => {
                    //check if both are eval, if so insert both
                    if eval.contains_key(x) && eval.contains_key(y) {
                        let new = fork_eval(
                            eval.get(x).unwrap(),
                            eval.get(y).unwrap(),
                        );
                        eval.insert(*index, new);
                    }
                }
                Rule::OrDual(x, y, w, z) => {
                    //if eval, combine and insert both
                    if eval.contains_key(x)
                        && eval.contains_key(y)
                        && eval.contains_key(w)
                        && eval.contains_key(z)
                    {
                        let a = combine_eval(
                            eval.get(x).unwrap(),
                            eval.get(y).unwrap(),
                        );
                        let b = combine_eval(
                            eval.get(w).unwrap(),
                            eval.get(z).unwrap(),
                        );
                        let new = fork_eval(&a, &b);
                        eval.insert(*index, new);
                    }
                }
            }
        }
        if eval.len() == number_rules {
            break;
        }
    }
    //count number of messages on 0
    let valid = eval.get(&0).unwrap();
    Ok(input.messages.iter().filter(|x| valid.contains(x)).count())
}

fn solve2(input: &Input) -> Result<usize, Box<Error>> {
    unimplemented!();
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
fn test_1() -> Result<(), Box<Error>> {
    const INPUT: &[(&str, Rule)] = &[
        ("4 1", Rule::Dual(4, 1)),
        ("2 | 3", Rule::Or(2, 3)),
        ("1 1 | 3 414", Rule::OrDual(1, 1, 3, 414)),
        ("123123", Rule::Alias(123123)),
        ("\"g\"", Rule::Value('g')),
        ("2340922039  999999999", Rule::Dual(2340922039, 999999999)),
    ];
    for (input, res) in INPUT.iter() {
        let input = input.parse::<Rule>()?;
        assert_eq!(input, *res);
    }
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Box<Error>> {
    const INPUT: &str = "0: 4 6
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
6: 1 5

ababbb
bababa
abbbab
aaabbb
aaaabbb ";
    let input = INPUT.parse()?;
    assert_eq!(solve1(&input)?, 2);
    Ok(())
}
