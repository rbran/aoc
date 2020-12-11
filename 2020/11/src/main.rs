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
}

fn main() -> Result<(), Box<Error>> {
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut ferry = Ferry::new(&input)?;
    println!("P1: {}", ferry.solve1());
    Ok(())
}

#[test]
fn test0() -> Result<(), Box<Error>> {
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
    let mut ferry = Ferry::new(INPUT)?;
    assert_eq!(ferry.solve1(), 37);
    Ok(())
}
