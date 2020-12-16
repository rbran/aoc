//I HATE everthing about this code, this is pure shit.
//But I was too tired to fix anything this day.
//Don't read bellow this line unless you are very motivated to fix lots of shit

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

fn find_valid_rule(field: usize, rules: &Vec<Rule>) -> Option<usize> {
    rules.iter().enumerate().find_map(|(i, rule)| {
        if (field >= rule.range1.0 && field <= rule.range1.1)
            || (field >= rule.range2.0 && field <= rule.range2.1) {
            Some(i)
        } else {
            None
        }
    })
}

// TODO: Return Iterator
fn find_all_valid_rules(field: usize, rules: &Vec<Rule>) -> Vec<usize> {
    rules.iter().enumerate().filter_map(|(i, rule)| {
        if (field >= rule.range1.0 && field <= rule.range1.1)
            || (field >= rule.range2.0 && field <= rule.range2.1) {
            Some(i)
        } else {
            None
        }
    }).collect()
}

fn and_vector(a: &mut Vec<usize>, b: &Vec<usize>) {
    let mut ret = Vec::new();
    for i in a.iter() {
        if a.contains(i) && b.contains(i) {
            ret.push(*i);
        }
    }
    a.clear();
    a.extend(ret);
}

fn solve2(input: &str) -> Result<usize, Box<Error>> {
    let input = input.parse::<Input>()?;
    let rules = input.rules;
    // remove invalid tickets
    let mut sudoku = Vec::new();
    sudoku.push(&input.your_ticket);
    for ticket in input.nearby_tickets.iter() {
        let first_valid = ticket.iter().find(|&field| {
            find_valid_rule(*field, &rules).is_none()
        });
        if first_valid.is_none() {
            sudoku.push(ticket);
        }
    }

    // prepare the sudoku shit
    let paper_lines = sudoku.len();
    let paper_columns = rules.len();
    //possible rules of each column
    let mut paper: Vec<Vec<Vec<usize>>> = vec![];
    for line in sudoku {
        let mut paper_line = vec![];
        for (c, field) in line.iter().enumerate() {
            let valid = find_all_valid_rules(*field, &rules);
            paper_line.push(valid);
        }
        paper.push(paper_line);
    }

    // do sudoku shit
    // values found
    let mut res: Vec<Option<usize>> = vec![None; rules.len()];
    loop {
        //clean the paper
        for &found in res.iter() {
            let found = match found {
                None => continue,
                Some(found) => found,
            };
            for line in 0..paper_lines {
                for col in 0..paper_columns {
                    if res[col].is_some() {
                        continue;
                    }
                    let remove = paper[line][col].iter().enumerate().find(|(i, x)| **x == found);
                    match remove {
                        None => (),
                        Some((i, _)) => {paper[line][col].remove(i);},
                    }
                }
            }
        }

        let mut change = false;

        //apply AND to all the columns
        for col in 0..paper_columns {
            if res[col].is_some() {
                continue;
            }
            let mut new_col = paper[0][col].clone();
            for line in 1..paper_lines {
                and_vector(&mut new_col, &paper[line][col]);
            }
            if new_col.len() == 1 {
                //found the solution for the col
                change = true;
                *res.get_mut(col).unwrap() = new_col.get(0).copied();
                for line in 1..paper_lines {
                    paper[line][col].clear();
                }
            } else {
                for line in 1..paper_lines {
                    if paper[line][col].len() != new_col.len() {
                        change = true;
                        *paper[line].get_mut(col).unwrap() = new_col.clone();
                    }
                }
            }
        }
        if change {
            continue; //clean and do again
        }
        change = false;

        //check if a line have only one rule that a apply to only one column
        for line in 0..paper_lines {
            let mut count = HashMap::new();
            for col in 0..paper_columns {
                if res[col].is_some() {
                    continue;
                }
                for field in paper[line][col].iter() {
                    *count.entry(field).or_insert(0) += 1;
                }
            }
            //check if a rule can only be applied in one place
            for (_, v) in count.iter() {
                if *v == 1 {
                    let (col, _) = paper[line].iter().enumerate().find(|(i, x)| x.iter().find(|y| **y == *v).is_some()).unwrap();
                    change = true;
                    *res.get_mut(col).unwrap() = Some(*v);
                }
            }
        }

        if change {
            continue;
        } else {
            break
        }
    }
    println!("res {:?}", res);
    let mut easy_bro = Vec::new();
    for (index, fuck_this) in res.iter().enumerate() {
        if fuck_this.unwrap() < 6 {
            easy_bro.push(index);
        }
    }
    let mut res = 1;
    for easy in easy_bro {
        res *= input.your_ticket[easy];
    }
    Ok(res)
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
