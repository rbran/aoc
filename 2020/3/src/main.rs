//the default input line len is 32, so is more eficient to get the value,
//I could use the input data raw, but instead I'll use this unecessary beautiful
//Vector, is not very eficient, but I like the rust interator working where

use std::env;
use std::fs;

#[derive(Clone)]
struct Slope<'a> {
    num_rows: usize,
    num_lines: usize,
    data: &'a [u8],
}

struct SlopeIter<'a> {
    mov: (usize, usize), //(x, y)
    pos: (usize, usize),
    slope: &'a Slope<'a>,
}

impl<'a> Slope<'a> {
    fn new(data: &'a [u8]) -> Slope {
        //get the input len of the lines and columns
        let num_rows = data.iter().position(|&x| x == b'\n').unwrap();
        assert!(num_rows > 0);
        let num_lines = data.len() / (num_rows + 1); //+1 because of \n

        //return the struct
        Slope {
            num_rows,
            num_lines,
            data,
        }
    }
}

impl<'a> SlopeIter<'a> {
    fn new(mov: (usize, usize), slope: &'a Slope<'a>) -> Self {
        SlopeIter {
            mov,
            pos: (0, 0),
            slope,
        }
    }
}

impl<'a> Iterator for SlopeIter<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.1 >= self.slope.num_lines {
            return None; //end of the slope
        }
        //calculate the current possition on data
        let pos = (self.pos.1 * (self.slope.num_rows + 1)) + self.pos.0;

        //calculate the next posistion
        self.pos.0 += self.mov.0;
        self.pos.1 += self.mov.1;
        //rows just repeats, so warp if necessary
        if self.pos.0 >= self.slope.num_rows {
            self.pos.0 %= self.slope.num_rows;
        }
        Some(self.slope.data[pos] == b'#')
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // assuming all passwords are lower case
    let input = fs::read(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let slope = Slope::new(&input);

    let p1_res = SlopeIter::new((3, 1), &slope).filter(|&x| x).count();
    println!("Part1: Found {} trees", p1_res);

    let mut p2_res = 1;
    for mov in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let res = SlopeIter::new(*mov, &slope).filter(|&x| x).count();
        p2_res *= res;
    }
    println!("Part2: Mult result {}", p2_res);
    Ok(())
}
