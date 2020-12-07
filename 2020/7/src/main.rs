//lets work with bytes instead of strings on this one
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

struct AviationRegulation<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> AviationRegulation<'a> {
    fn new(data: &'a str) -> Self {
        let rules_raw = data.lines();
        let mut rules = Vec::new();
        for rule_raw in rules_raw {
            let rule = Rule::new(rule_raw);
            rules.push(rule);
        }
        AviationRegulation { rules }
    }

    fn solve_p1(&self, bag: &str) -> usize {
        let mut ret = 0;
        let mut bags_check = vec![bag];
        let mut bags_checked = vec![]; //need in case we have a cycle
        loop {
            let bag = match bags_check.pop() {
                Some(x) => x,
                None => break, //no more bags to check
            };
            bags_checked.push(bag);
            for rule in &self.rules {
                if rule.allowed.contains_key(bag) {
                    if !bags_checked.contains(&rule.bag) && !bags_check.contains(&rule.bag) {
                        //our bag can fit inside this bag, so check where this fit
                        ret += 1;
                        bags_check.push(rule.bag);
                    }
                }
            }
        }
        ret
    }
}

struct Rule<'a> {
    bag: &'a str,
    allowed: HashMap<&'a str, usize>,
}

impl<'a> Rule<'a> {
    fn new(data: &'a str) -> Self {
        lazy_static! {
            static ref RE_NAME: Regex = Regex::new("(\\w+ \\w+) bags contain ").unwrap();
            static ref RE_BAGS: Regex = Regex::new("(\\d+) (\\w+ \\w+) bags?[\\.,]").unwrap();
        }
        let bag_name = match RE_NAME.captures(data) {
            Some(x) => x.get(1).unwrap().as_str(),
            None => panic!("Bag name not found"),
        };
        let mut allowed = HashMap::new();
        for caps in RE_BAGS.captures_iter(data) {
            let number = caps.get(1).unwrap().as_str();
            let number = number.parse::<usize>().unwrap();
            let bag = caps.get(2).unwrap().as_str();

            //check if bag is already allowed
            assert_eq!(allowed.get(bag), None);
            allowed.insert(bag, number);
        }
        Rule {
            bag: bag_name,
            allowed,
        }
    }
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read_to_string(file_path).expect("Unable to read the File");
    let rules = AviationRegulation::new(file.as_str());
    println!("P1: {}", rules.solve_p1("shiny gold"));
}
