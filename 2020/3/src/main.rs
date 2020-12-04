//the default input line len is 32, so is more eficient to get the value,
//I could use the input data raw, but instead I'll use this unecessary beautiful
//Vector, is not very eficient, but I like the rust interator working where

use std::env;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

#[derive(Clone)]
struct Slope {
    mov_x: usize,
    mov_y: usize,
    pos_x: usize,
    pos_y: usize,
    data: Vec<Vec<bool>>, //tree is true
}

impl Slope {
    fn new(raw_data: Vec<u8>) -> Slope {
        //get the input len of the lines and columns
        let row_len = raw_data
            .iter()
            .position(|&x| x == b'\n')
            .expect("Didn't found \\n on input data");
        assert!(row_len > 0);
        let line_len = raw_data.len() / (row_len + 1); //+1 because of \n

        //convert the input from bytes to true/false
        let mut input = raw_data.iter().filter_map(|x| match *x {
            b'.' => Some(false),
            b'#' => Some(true),
            b'\n' => None, //ignore new line
            _ => panic!("Invalid input data"),
        });

        //create a vector that will store all lines
        let mut data = Vec::with_capacity(line_len);
        for _ in 0..line_len {
            //the line vector
            let mut row = Vec::with_capacity(row_len);
            for _ in 0..row_len {
                row.push(input.next().expect("Invalid input size"));
            }
            data.push(row);
        }
        assert_eq!(input.next(), None); //just confirm the EoF

        //return the struct
        Slope {
            mov_x: 3,
            mov_y: 1,
            pos_x: 0,
            pos_y: 0,
            data,
        }
    }

    fn reset(&mut self, mov_x: usize, mov_y: usize) {
        self.mov_x = mov_x;
        self.mov_y = mov_y;
        self.pos_x = 0;
        self.pos_y = 0;
    }
}

impl Iterator for Slope {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_y >= self.data.len() {
            return None; // reach the end of the slope
        }

        let ret = self.data[self.pos_y][self.pos_x];
        self.pos_x += self.mov_x;
        self.pos_y += self.mov_y;

        if self.pos_x >= self.data[0].len() {
            // slope just repeat to the right, so go back to the start
            self.pos_x %= self.data[0].len();
        }
        Some(ret)
    }
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .or(Some(DEFAULT_INPUT_FILE.to_string()))
        .unwrap();
    let input_data = fs::read(file_path).expect("Unable to read the File");
    let slope = Slope::new(input_data);
    let p1_res = slope.clone().filter(|&x| x).count();
    //TODO: Fix this clone implementing IntoIterator
    println!("Part1: Found {} trees", p1_res);

    let mut p2_res = 1;
    let p2_tests: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (mov_x, mov_y) in p2_tests.iter() {
        let mut slope = slope.clone();
        slope.reset(*mov_x, *mov_y);
        let res = slope.filter(|&x| x).count();
        p2_res *= res;
    }
    println!("Part2: Mult result {}", p2_res);
}
