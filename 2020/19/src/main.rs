//the solution is ok, but there is too much memory copy

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

fn solve1(input: &Input) -> Result<usize, Box<Error>> {
    let mut ret = 0;
    for message in input.messages.iter() {
        let mut chars = message.chars();
        let matchs = check_rule(&input.rules, &mut chars, input.rules.get(&0).unwrap());
        if let Some(matchs) = matchs {
            for part_match in matchs {
                if message.len() == part_match {
                    ret += 1;
                }
            }
        }
    }
    Ok(ret)
}

fn check_rule(
    rules: &HashMap<usize, Rule>,
    rest_message: &mut std::str::Chars,
    current_rule: &Rule,
) -> Option<Vec<usize>> {
    match current_rule {
        Rule::Value(value) => match rest_message.next() {
            Some(check) if check == *value => Some(vec![1]),
            _ => None,
        },
        Rule::Alias(x) => {
            check_rule(rules, rest_message, rules.get(x).unwrap())
        }
        Rule::Dual(x, y) => {
            let matchs1 = check_rule(
                rules,
                &mut rest_message.clone(),
                rules.get(x).unwrap(),
            )?;
            let rule2 = rules.get(y).unwrap();
            let mut ret_matchs = vec![];
            for match1 in matchs1.iter() {
                let mut message = rest_message.clone();
                //TODO: WHAT ARE THOSE?!?!?!?!?, use generics instead
                for _ in 0..*match1 {
                    message.next();
                }
                let match2 = check_rule(rules, &mut message, rule2);
                if let Some(match2) = match2 {
                    ret_matchs
                        .extend(match2.iter().map(|match2| *match2 + *match1));
                }
            }
            if ret_matchs.len() == 0 {
                return None;
            } else {
                return Some(ret_matchs);
            }
        }
        Rule::Or(x, y) => {
            let match1 = check_rule(
                rules,
                &mut rest_message.clone(),
                rules.get(x).unwrap(),
            );
            let match2 = check_rule(rules, rest_message, rules.get(y).unwrap());
            return match (match1, match2) {
                (None, None) => None,
                (None, Some(match2)) => Some(match2),
                (Some(match1), None) => Some(match1),
                (Some(mut match1), Some(mut match2)) => {
                    match1.append(&mut match2);
                    Some(match1)
                }
            };
        }
        Rule::OrDual(x, y, z, w) => {
            let match1 = check_rule(
                rules,
                &mut rest_message.clone(),
                &Rule::Dual(*x, *y),
            );
            let match2 = check_rule(rules, rest_message, &Rule::Dual(*z, *w));
            return match (match1, match2) {
                (None, None) => None,
                (None, Some(match2)) => Some(match2),
                (Some(match1), None) => Some(match1),
                (Some(mut match1), Some(mut match2)) => {
                    match1.append(&mut match2);
                    Some(match1)
                }
            };
        }
    }
}

fn solve2(input: &Input) -> Result<usize, Box<Error>> {
    let mut rules = input.rules.clone();
    //add the rules
    //x: 42 8
    //y: 42 11
    //8: 42 | x
    //11: 42 31 | y 31
    let pos_x = rules.len();
    let pos_y = pos_x + 1;
    rules.insert(pos_x, Rule::Dual(42, 8));
    rules.insert(pos_y, Rule::Dual(42, 11));
    rules.insert(8, Rule::Or(42, pos_x));
    rules.insert(11, Rule::OrDual(42, 31, pos_y, 31));

    let mut ret = 0;
    for message in input.messages.iter() {
        let mut chars = message.chars();
        let matchs = check_rule(&rules, &mut chars, rules.get(&0).unwrap());
        if let Some(matchs) = matchs {
            for part_match in matchs {
                if message.len() == part_match {
                    ret += 1;
                }
            }
        }
    }
    Ok(ret)
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
aaaabbb";
    let input = INPUT.parse()?;
    assert_eq!(solve1(&input)?, 2);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), Box<Error>> {
    const INPUT: &str = "0: 4 6
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
6: 1 5

a";
    let input = INPUT.parse::<Input>()?;
    const MESSAGES: &[(&str, bool)] = &[
        ("ababbb", true),
        ("bababa", false),
        ("abbbab", true),
        ("aaabbb", false),
        ("aaaabbb", false),
    ];
    for (message, res) in MESSAGES.iter() {
        let result = check_rule(
            &input.rules,
            &mut message.chars(),
            input.rules.get(&0).unwrap(),
        );
        match result {
            None => assert!(!*res),
            Some(x) => {
                let mut ret = false;
                for part_match in x {
                    if message.len() == part_match {
                        ret = true;
                        break;
                    }
                }
                assert_eq!(ret, *res);
            }
        }
    }
    Ok(())
}
