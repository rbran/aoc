use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

struct Rule {
    name: String,
    range1: (usize, usize),
    range2: (usize, usize),
}

impl FromStr for Rule {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = s.split(':');
        let err = || Error::new(InvalidData, "Invalid Size");
        let name = inputs.next().ok_or_else(err)?;
        let value = inputs.next().ok_or_else(err)?;
        let mut inputs = value.trim().split(" or ");
        let mut range1 = inputs.next().ok_or_else(err)?.split('-');
        let mut range2 = inputs.next().ok_or_else(err)?.split('-');

        let range1_min = range1.next().ok_or_else(err)?;
        let range1_max = range1.next().ok_or_else(err)?;
        let range2_min = range2.next().ok_or_else(err)?;
        let range2_max = range2.next().ok_or_else(err)?;
        let err = |_| Err(Error::new(InvalidData, "Unable to Parse range"));
        let range1_min = range1_min.parse::<usize>().or_else(err)?;
        let range1_max = range1_max.parse::<usize>().or_else(err)?;
        let range2_min = range2_min.parse::<usize>().or_else(err)?;
        let range2_max = range2_max.parse::<usize>().or_else(err)?;
        if name.len() == 0 {
            Err(Box::new(Error::new(InvalidData, "Invalid Format")))
        } else {
            Ok(Rule {
                name: name.to_string(),
                range1: (range1_min, range1_max),
                range2: (range2_min, range2_max),
            })
        }
    }
}

struct Input {
    rules: Vec<Rule>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

fn parse_ticket(line: &str) -> Result<Vec<usize>, Box<Error>> {
    let ticket_fields = line;
        let ticket = ticket_fields
            .split(',')
            .map(|field| field.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .or_else(|_| Err(Error::new(InvalidData, "")))?;
        if ticket.len() == 0 {
            return Err(Box::new(Error::new(InvalidData, "")));
        }
        Ok(ticket)
}

impl FromStr for Input {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut rules = Vec::new();
        for line in &mut lines {
            if line.len() == 0 {
                //end rules and start your ticket
                break;
            }
            rules.push(line.parse::<Rule>()?);
        }
        let err = || Error::new(InvalidData, "");
        //parse your ticket
        let title_your_ticket = lines.next().ok_or_else(err)?;
        if title_your_ticket != "your ticket:" {
            return Err(Box::new(Error::new(InvalidData, "")));
        }
        let your_ticket = lines.next().ok_or_else(err)?;
        let your_ticket = parse_ticket(your_ticket)?;

        //parse neaby tickets
        if lines.next().ok_or_else(err)?.len() != 0 {
            return Err(Box::new(Error::new(InvalidData, "")));
        }
        let title_nearby_tickets = lines.next().ok_or_else(err)?;
        if title_nearby_tickets != "nearby tickets:" {
            return Err(Box::new(Error::new(InvalidData, "")));
        }
        let nearby_tickets = lines.map(|line| {
            parse_ticket(line)
        }).collect::<Result<Vec<_>, _>>()?;
        if nearby_tickets.len() == 0 {
            return Err(Box::new(Error::new(InvalidData, "")));
        }
        Ok(Input {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

fn solve1(input: &str) -> Result<usize, Box<Error>> {
    let input = input.parse::<Input>()?;
    let ret = input.nearby_tickets.iter().map(|ticket| {
        ticket.iter().map(|&field| {
            input.rules.iter().find_map(|rule| {
                if (field >= rule.range1.0 && field <= rule.range1.1)
                    || (field >= rule.range2.0 && field <= rule.range2.1) {
                        Some(0)
                } else {
                    None
                }
            }).unwrap_or(field)
        }).sum::<usize>()
    }).sum();
    Ok(ret)
}

fn solve2(input: &str) -> Result<usize, Box<Error>> {
    unimplemented!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    println!("P1: {}", solve1(&input)?);
    println!("P2: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Box<Error>> {
    const INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    assert_eq!(solve1(INPUT)?, 71);
    Ok(())
}
