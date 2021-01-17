use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

struct Input {
    dirs: Vec<Vec<Dir>>,
}

fn error(s: &str) -> Err {
    Box::new(Error::new(InvalidData, s.to_string()))
}

impl FromStr for Input {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dirs = Vec::new();
        for line in s.lines() {
            let mut dir = Vec::new();
            let mut chars = line.chars();
            loop {
                let add = match chars.next() {
                    None => break,
                    Some('e') => Dir::E,
                    Some('w') => Dir::W,
                    Some('s') => match chars.next() {
                        Some('e') => Dir::SE,
                        Some('w') => Dir::SW,
                        _ => return Err(error("invalid input")),
                    },
                    Some('n') => match chars.next() {
                        Some('e') => Dir::NE,
                        Some('w') => Dir::NW,
                        _ => return Err(error("invalid input")),
                    },
                    _ => return Err(error("Invalid input")),
                };
                dir.push(add);
            }

            dirs.push(dir);
        }
        Ok(Input { dirs })
    }
}

//The grid coord
//    __    __     E__
//   /22\__/20\  NE/XY\SE
//   \__/11\__/  NW\__/SW
//   /12\__/10\     W
//   \__/01\__/
//   /02\__/00\
//   \__/  \__/
struct Part1<'a> {
    input: &'a Input,
    floor: HashSet<(isize, isize)>,
}

impl<'a> TryFrom<&'a Input> for Part1<'a> {
    type Error = Err;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        Ok(Self {
            input,
            floor: HashSet::new(),
        })
    }
}

impl<'a> Part1<'a> {
    fn solve(&mut self) -> Result<usize, Err> {
        //if the exists is black, otherwise is white
        for dir in self.input.dirs.iter() {
            let mut coord = (0, 0);
            for side in dir.iter() {
                match side {
                    Dir::E => coord.0 += 1,
                    Dir::W => coord.0 -= 1,
                    Dir::NE => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0, coord.1 + 1)
                        } else {
                            coord = (coord.0 + 1, coord.1 + 1)
                        }
                    }
                    Dir::SE => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0, coord.1 - 1)
                        } else {
                            coord = (coord.0 + 1, coord.1 - 1)
                        }
                    }
                    Dir::NW => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0 - 1, coord.1 + 1)
                        } else {
                            coord = (coord.0, coord.1 + 1)
                        }
                    }
                    Dir::SW => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0 - 1, coord.1 - 1)
                        } else {
                            coord = (coord.0, coord.1 - 1)
                        }
                    }
                }
            }
            if self.floor.contains(&coord) {
                //if black, flip to white
                self.floor.remove(&coord);
            } else {
                //if white, flip to black
                self.floor.insert(coord);
            }
        }
        Ok(self.floor.len())
    }
}

struct Part2 {
    floor: HashSet<(isize, isize)>,
}

impl<'a> TryFrom<&Part1<'a>> for Part2 {
    type Error = Err;
    fn try_from(part1: &Part1) -> Result<Self, Self::Error> {
        let mut floor = HashSet::new();
        for k in part1.floor.iter() {
            floor.insert(*k);
        }
        Ok(Part2 { floor })
    }
}

impl Part2 {
    fn get_all_around(x: &(isize, isize)) -> [(isize, isize); 7] {
        let (x, y) = *x;
        [
            (x, y),                                      //self
            (x + 1, y),                                  //E
            (x - 1, y),                                  //W
            (x + if y % 2 == 0 { 0 } else { 1 }, y + 1), //NE
            (x + if y % 2 == 0 { 0 } else { 1 }, y - 1), //SE
            (x - if y % 2 == 0 { 1 } else { 0 }, y + 1), //NW
            (x - if y % 2 == 0 { 1 } else { 0 }, y - 1), //SW
        ]
    }

    fn solve(&mut self) -> Result<usize, Err> {
        //the value is false to white, true to black
        let mut floor_cal = HashMap::new();
        for _ in 0..100 {
            //process only the black tiles and neighbours
            for hex_black in self.floor.iter() {
                //check the hex and the 6 neigbours
                for hex in Self::get_all_around(hex_black).iter() {
                    //check if this hex was already processed
                    if floor_cal.contains_key(hex) {
                        continue;
                    }
                    //check how many neibours are black
                    let mut found = 0;
                    for nei in Self::get_all_around(hex).iter().skip(1) {
                        if self.floor.contains(nei) {
                            found += 1;
                        }
                    }
                    let color = if self.floor.contains(hex) {
                        //black, flip if neig == 0 or > 2
                        !(found == 0 || found > 2)
                    } else {
                        //white, flip if nei eq 2 black
                        found == 2
                    };
                    floor_cal.insert(*hex, color);
                }
            }
            self.floor.clear();
            for (k, v) in floor_cal.iter() {
                if *v {
                    self.floor.insert(*k);
                }
            }
            floor_cal.clear();
        }
        Ok(self.floor.len())
    }
}

fn main() -> Result<(), Err> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    println!("P1: {}", part1.solve()?);
    let mut part2 = Part2::try_from(&part1)?;
    println!("P2: {}", part2.solve()?);
    Ok(())
}

#[test]
fn test_example() -> Result<(), Err> {
    const INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let input: Input = INPUT.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    assert_eq!(part1.solve()?, 10);
    let mut part2 = Part2::try_from(&part1)?;
    assert_eq!(part2.solve()?, 2208);
    Ok(())
}
