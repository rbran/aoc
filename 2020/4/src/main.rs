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
    for raw_doc in raw_docs {
        let doc = Doc::new(raw_doc);
        if doc.is_valid_p1() {
            p1_res += 1;
        }
    }
    println!("P1: {}", p1_res);
}
