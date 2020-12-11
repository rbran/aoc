// Other data modeling focus solution that gone wild
// Ps this is obvious a game-of-life problem, I should had read about the game

use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

enum Space {
    Seat(bool),
    Floor,
}

impl Space {
    fn new(data: char) -> Result<Self, Box<Error>> {
        match data {
            'L' => Ok(Space::Seat(false)),
            '#' => Ok(Space::Seat(true)),
            '.' => Ok(Space::Floor),
            _ => Err(Box::new(Error::new(InvalidData, "Unkown input"))),
        }
    }

    fn seat(&self) -> Option<bool> {
        match *self {
            Space::Seat(x) => Some(x),
            _ => None,
        }
    }
}

struct Ferry {
    spaces: Vec<Vec<Space>>,
    x_len: usize,
    y_len: usize,
}

impl Ferry {
    fn new(data: &str) -> Result<Self, Box<Error>> {
        let x_len = data
            .find('\n')
            .ok_or_else(|| Error::new(InvalidData, "Unable to find new line"))?;
        let spaces: Vec<_> = data
            .lines()
            .map(|x| {
                let line: Vec<_> = x.chars().map(|x| Space::new(x)).collect::<Result<_, _>>()?;
                if line.len() != x_len {
                    Err(Box::new(Error::new(InvalidData, "Invalid line len")))
                } else {
                    Ok(line)
                }
            })
            .collect::<Result<_, _>>()?;
        let y_len = spaces.len();
        Ok(Ferry {
            spaces,
            x_len,
            y_len,
        })
    }

    fn occupied_adjacent(&self, pos_x: usize, pos_y: usize) -> usize {
        if pos_x >= self.x_len || pos_y >= self.y_len {
            panic!("Invalid location");
        }
        let has_left = pos_x > 0; //not the first on the left
        let has_right = pos_x < self.x_len - 1; //not the last on the right
        let has_up = pos_y > 0;
        let has_down = pos_y < self.y_len - 1;

        let occupied = |pos_x: usize, pos_y: usize| match self.spaces[pos_y][pos_x] {
            Space::Seat(true) => 1,
            _ => 0,
        };

        //pos 1 2 3
        //    4 X 5
        //    6 7 8
        let mut ret = 0;
        if has_left {
            ret += occupied(pos_x - 1, pos_y); //pos 4
            if has_up {
                ret += occupied(pos_x - 1, pos_y - 1); //pos 1
            }
            if has_down {
                ret += occupied(pos_x - 1, pos_y + 1); //pos 6
            }
        }
        if has_right {
            ret += occupied(pos_x + 1, pos_y); //pos 5
            if has_up {
                ret += occupied(pos_x + 1, pos_y - 1); //pos 3
            }
            if has_down {
                ret += occupied(pos_x + 1, pos_y + 1); //pos 8
            }
        }
        if has_up {
            ret += occupied(pos_x, pos_y - 1); //pos 2
        }
        if has_down {
            ret += occupied(pos_x, pos_y + 1); //pos 7
        }
        ret
    }

    fn process1(&mut self) -> bool {
        let mut changed = false;
        let mut new_state = Vec::with_capacity(self.y_len);

        for (pos_y, line) in self.spaces.iter().enumerate() {
            let mut new_line = Vec::with_capacity(self.x_len);
            for (pos_x, seat) in line.iter().enumerate() {
                let new_seat = match seat {
                    Space::Seat(false) => {
                        // If a seat is empty (L) and there are no
                        // occupied seats adjacent to it, the seat
                        // becomes occupied.
                        let occupied = self.occupied_adjacent(pos_x, pos_y);
                        if occupied == 0 {
                            changed = true;
                            Space::Seat(true)
                        } else {
                            Space::Seat(false)
                        }
                    }
                    Space::Seat(true) => {
                        // If a seat is occupied (#) and four or more
                        // seats adjacent to it are also occupied, the
                        // seat becomes empty.
                        let occupied = self.occupied_adjacent(pos_x, pos_y);
                        if occupied >= 4 {
                            changed = true;
                            Space::Seat(false)
                        } else {
                            Space::Seat(true)
                        }
                    }
                    Space::Floor => Space::Floor,
                };
                new_line.push(new_seat);
            }
            new_state.push(new_line);
        }

        self.spaces = new_state;

        changed
    }

    fn occupied_total(&self) -> usize {
        self.spaces
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|&seat| match *seat {
                        Space::Seat(true) => true,
                        _ => false,
                    })
                    .count()
            })
            .sum()
    }

    fn solve1(&mut self) -> usize {
        while self.process1() {}
        self.occupied_total()
    }

    fn occupied_view(&self, pos_x: usize, pos_y: usize) -> usize {
        if pos_x >= self.x_len || pos_y >= self.y_len {
            panic!("Invalid location");
        }
        let len_left = pos_x;
        let len_right = self.x_len - 1 - pos_x;
        let len_up = pos_y;
        let len_down = self.y_len - 1 - pos_y;

        //dir 1 2 3
        //    4 X 5
        //    6 7 8
        let mut ret = 0;
        //dir 4
        if len_left > 0 {
            ret += self.spaces[pos_y][..pos_x]
                .iter()
                .rev()
                .find_map(|x| x.seat())
                .unwrap_or(false) as usize;
            //dir 1
            if len_up > 0 {
                let size = len_up.min(len_left);
                ret += self.spaces[pos_y - size..pos_y]
                    .iter()
                    .rev()
                    .enumerate()
                    .find_map(|(mov_y, line)| line[pos_x - 1 - mov_y].seat())
                    .unwrap_or(false) as usize;
            }
            //dir 6
            if len_down > 0 {
                let size = len_down.min(len_left);
                ret += self.spaces[pos_y + 1..pos_y + 1 + size]
                    .iter()
                    .enumerate()
                    .find_map(|(mov_y, line)| line[pos_x - 1 - mov_y].seat())
                    .unwrap_or(false) as usize;
            }
        }
        //dir 5
        if len_right > 0 {
            ret += self.spaces[pos_y][pos_x + 1..]
                .iter()
                .find_map(|x| x.seat())
                .unwrap_or(false) as usize;
            //dir 3
            if len_up > 0 {
                let size = len_up.min(len_right);
                ret += self.spaces[pos_y - size..pos_y]
                    .iter()
                    .rev()
                    .enumerate()
                    .find_map(|(mov_y, line)| line[pos_x + 1 + mov_y].seat())
                    .unwrap_or(false) as usize;
            }
            //dir 8
            if len_down > 0 {
                let size = len_down.min(len_right);
                ret += self.spaces[pos_y + 1..pos_y + 1 + size]
                    .iter()
                    .enumerate()
                    .find_map(|(mov_y, line)| line[pos_x + 1 + mov_y].seat())
                    .unwrap_or(false) as usize;
            }
        }
        //dir 2
        if len_up > 0 {
            ret += self.spaces[..pos_y]
                .iter()
                .rev()
                .find_map(|x| x[pos_x].seat())
                .unwrap_or(false) as usize;
        }

        //dir 7
        if len_down > 0 {
            ret += self.spaces[pos_y + 1..]
                .iter()
                .find_map(|x| x[pos_x].seat())
                .unwrap_or(false) as usize;
        }
        ret
    }

    fn process2(&mut self) -> bool {
        let mut changed = false;
        let mut new_state = Vec::with_capacity(self.y_len);

        for (pos_y, line) in self.spaces.iter().enumerate() {
            let mut new_line = Vec::with_capacity(self.x_len);
            for (pos_x, seat) in line.iter().enumerate() {
                let new_seat = match seat {
                    Space::Seat(false) => {
                        // The other rules still apply: empty seats that see no
                        // occupied seats become occupied
                        let occupied = self.occupied_view(pos_x, pos_y);
                        if occupied == 0 {
                            changed = true;
                            Space::Seat(true)
                        } else {
                            Space::Seat(false)
                        }
                    }
                    Space::Seat(true) => {
                        // it now takes five or more visible occupied seats for
                        // an occupied seat to become empty
                        let occupied = self.occupied_view(pos_x, pos_y);
                        if occupied >= 5 {
                            changed = true;
                            Space::Seat(false)
                        } else {
                            Space::Seat(true)
                        }
                    }
                    Space::Floor => Space::Floor,
                };
                new_line.push(new_seat);
            }
            new_state.push(new_line);
        }

        self.spaces = new_state;

        changed
    }

    fn solve2(&mut self) -> usize {
        while self.process2() {}
        self.occupied_total()
    }
}

fn main() -> Result<(), Box<Error>> {
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut ferry = Ferry::new(&input)?;
    println!("P1: {}", ferry.solve1());
    let mut ferry = Ferry::new(&input)?;
    println!("P2: {}", ferry.solve2());
    Ok(())
}

#[allow(dead_code)]
const INPUT: &str = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";

#[test]
fn test0() -> Result<(), Box<Error>> {
    let mut ferry = Ferry::new(INPUT)?;
    assert_eq!(ferry.solve1(), 37);
    Ok(())
}

#[test]
fn test1() -> Result<(), Box<Error>> {
    let mut ferry = Ferry::new(INPUT)?;
    assert_eq!(ferry.solve2(), 26);
    Ok(())
}

#[test]
fn test2() -> Result<(), Box<Error>> {
    const INPUT2: &str = ".......#.\n\
                          ...#.....\n\
                          .#.......\n\
                          .........\n\
                          ..#L....#\n\
                          ....#....\n\
                          .........\n\
                          #........\n\
                          ...#.....";
    let ferry = Ferry::new(INPUT2)?;
    assert_eq!(ferry.occupied_view(3, 4), 8);
    Ok(())
}

#[test]
fn test3() -> Result<(), Box<Error>> {
    const INPUT2: &str = ".............\n\
                          .L.L.#.#.#.#.\n\
                          .............";
    let ferry = Ferry::new(INPUT2)?;
    assert_eq!(ferry.occupied_view(0, 1), 0);
    Ok(())
}

#[test]
fn test4() -> Result<(), Box<Error>> {
    const INPUT2: &str = ".##.##.\n\
                          #.#.#.#\n\
                          ##...##\n\
                          ...L...\n\
                          ##...##\n\
                          #.#.#.#\n\
                          .##.##.";
    let ferry = Ferry::new(INPUT2)?;
    assert_eq!(ferry.occupied_view(3, 3), 0);
    Ok(())
}
