//lets work with bytes instead of strings on this one
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

struct AviationRegulation<'a> {
    rules: HashMap<&'a str, Rc<RefCell<Rule<'a>>>>,
}

impl<'a> AviationRegulation<'a> {
    fn new(data: &'a str) -> Self {
        let rules_raw = data.lines();
        let mut rules = HashMap::new();
        for rule_raw in rules_raw {
            let rule_raw = Rule::new(rule_raw);
            let bag = rule_raw.bag;
            let rule = Rc::new(RefCell::new(rule_raw));
            rules.insert(bag, rule);
        }
        AviationRegulation {
            rules,
        }
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
            for (rule_bag, rule) in &self.rules {
                if rule.borrow().allowed.contains_key(bag) {
                    if !bags_checked.contains(&rule_bag) && !bags_check.contains(&rule_bag) {
                        //our bag can fit inside this bag, so check where this fit
                        ret += 1;
                        bags_check.push(rule_bag);
                    }
                }
            }
        }
        ret
    }

    fn solve_p2(&self, bag: &str) -> usize {
        let mut ret = 0;

        let mut rule = self.rules.get(bag).unwrap().borrow_mut();
        if let Some(x) = rule.contains {
            return x;
        }
        for (bag, number) in rule.allowed.iter() {
            // the bag contains all it's internal bags + itself
            ret += (self.solve_p2(bag) + 1) * number;
        }
        rule.contains = Some(ret);

        ret
    }
}

struct Rule<'a> {
    bag: &'a str,
    allowed: HashMap<&'a str, usize>,
    contains: Option<usize>,
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
            contains: None
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
    println!("P2: {}", rules.solve_p2("shiny gold"));
}
