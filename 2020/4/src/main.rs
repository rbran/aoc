//lets work with bytes instead of strings on this one
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::bytes::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

struct Doc<'a> {
    data: HashMap<&'a [u8], &'a [u8]>,
}

impl<'a> Doc<'a> {
    fn new(data: &'a [u8]) -> Self {
        let mut doc = Doc {
            data: HashMap::new(),
        };
        lazy_static! {
            static ref RE_FV: Regex = Regex::new("([a-z]{3}):([^\\s]+)").unwrap();
        }
        for caps in RE_FV.captures_iter(data) {
            let field = caps.get(1).unwrap().as_bytes();
            let value = caps.get(2).unwrap().as_bytes();
            if field.len() != 3 {
                panic!("Strange field len {}", caps[1].len());
            }
            //check if doc already have this field
            assert_eq!(doc.data.get(field), None);

            doc.data.insert(field, value);
        }
        doc
    }

    fn is_valid_p1(&self) -> bool {
        const REQ_FIELDS: &[&[u8]] = &[b"byr", b"iyr", b"eyr", b"hgt", b"hcl", b"ecl", b"pid"];
        for field in REQ_FIELDS.iter() {
            if let None = self.data.get(field) {
                return false;
            }
        }
        true
    }

    fn is_valid_p2(&self) -> bool {
        for (key, value) in self.data.iter() {
            match key {
                &b"byr" => {
                    if value.len() != 4 {
                        return false;
                    }
                    let value = String::from_utf8_lossy(value).parse::<u32>().unwrap_or(0);
                    if value > 2002 || value < 1920 {
                        return false;
                    }
                }
                &b"iyr" => {
                    if value.len() != 4 {
                        return false;
                    }
                    let value = String::from_utf8_lossy(value).parse::<u32>().unwrap_or(0);
                    if value > 2020 || value < 2010 {
                        return false;
                    }
                }
                &b"eyr" => {
                    if value.len() != 4 {
                        return false;
                    }
                    let value = String::from_utf8_lossy(value).parse::<u32>().unwrap_or(0);
                    if value > 2030 || value < 2020 {
                        return false;
                    }
                }
                &b"hgt" => {
                    lazy_static! {
                        static ref RE_HGT: Regex = Regex::new("([0-9]{2,3})(cm|in)").unwrap();
                    }
                    let cap = RE_HGT.captures(value);
                    if cap.is_none() {
                        return false;
                    }
                    let cap = cap.unwrap();
                    let value = String::from_utf8_lossy(cap.get(1).unwrap().as_bytes())
                        .parse::<u32>()
                        .unwrap_or(0);
                    let unit = cap.get(2).unwrap().as_bytes();
                    match unit {
                        b"cm" => {
                            if value < 150 || value > 193 {
                                return false;
                            }
                        }
                        b"in" => {
                            if value < 59 || value > 76 {
                                return false;
                            }
                        }
                        _ => panic!("WTF??"),
                    }
                }
                &b"hcl" => {
                    if value.len() != 7 {
                        return false;
                    }
                    lazy_static! {
                        static ref RE_HCL: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
                    }
                    if !RE_HCL.is_match(value) {
                        return false;
                    }
                }
                &b"ecl" => {
                    const ALLOWED: &[&[u8; 3]] =
                        &[b"amb", b"blu", b"brn", b"gry", b"grn", b"hzl", b"oth"];
                    if value.len() != 3 {
                        return false;
                    }
                    if ALLOWED.iter().find(|x| x.eq(&value)).is_none() {
                        return false;
                    }
                }
                &b"pid" => {
                    if value.len() != 9 {
                        return false;
                    }
                    for &i in value.iter() {
                        if i < b'0' || i > b'9' {
                            return false;
                        }
                    }
                }
                &b"cid" => (), // ignored
                _ => panic!("WTF?!?!"),
            }
        }
        true
    }
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let file = fs::read(file_path).expect("Unable to read the File");

    //will split the file into by '\n\n', so we get a iter for each doc &[u8]
    let mut new_doc_flag = false;
    let raw_docs = file.split(|&x| {
        if new_doc_flag {
            new_doc_flag = false;
            if x == b'\n' {
                return true;
            }
        } else {
            if x == b'\n' {
                new_doc_flag = true;
            }
        }
        false
    });

    //check the docs
    let mut p1_res = 0usize;
    let mut p2_res = 0usize;
    for raw_doc in raw_docs {
        let doc = Doc::new(raw_doc);
        if doc.is_valid_p1() {
            p1_res += 1;
            if doc.is_valid_p2() {
                p2_res += 1;
            }
        }
    }
    println!("P1: {}", p1_res);
    println!("P2: {}", p2_res);
}
