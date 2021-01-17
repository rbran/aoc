use std::collections::HashSet;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

enum Dir {
    E, SE, SW, W, NW, NE
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
                    Some('s') => {
                        match chars.next() {
                            Some('e') => Dir::SE,
                            Some('w') => Dir::SW,
                            _ => return Err(error("invalid input")),
                        }
                    },
                    Some('n') => {
                        match chars.next() {
                            Some('e') => Dir::NE,
                            Some('w') => Dir::NW,
                            _ => return Err(error("invalid input")),
                        }
                    },
                    _ => return Err(error("Invalid input")),
                };
                dir.push(add);
            }

            dirs.push(dir);
        }
        Ok(Input{dirs})
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
        Ok(Self{input,
        floor: HashSet::new()})
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
                    },
                    Dir::SE => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0, coord.1 - 1)
                        } else {
                            coord = (coord.0 + 1, coord.1 - 1)
                        }
                    },
                    Dir::NW => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0 - 1, coord.1 + 1)
                        } else {
                            coord = (coord.0, coord.1 + 1)
                        }
                    },
                    Dir::SW => {
                        if coord.1 % 2 == 0 {
                            coord = (coord.0 - 1, coord.1 - 1)
                        } else {
                            coord = (coord.0, coord.1 - 1)
                        }
                    },
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

struct Part2<'a> {
    input: &'a Input,
}

impl<'a> TryFrom<&'a Part1<'a>> for Part2<'a> {
    type Error = Err;
    fn try_from(part1: &'a Part1) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl<'a> Part2<'a> {
    fn solve(&mut self) -> Result<usize, Err> {
        unimplemented!()
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
