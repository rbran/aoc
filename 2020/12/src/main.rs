use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Debug)]
enum RotDir {
    L,
    F,
    R,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum MovDir {
    N,
    S,
    E,
    W,
}

impl MovDir {
    fn rotate(&self, dir: RotDir) -> Self {
        let num = match *self {
            MovDir::N => 0,
            MovDir::E => 1,
            MovDir::S => 2,
            MovDir::W => 3,
        };
        let rot = match dir {
            RotDir::L => 3,
            RotDir::F => 2,
            RotDir::R => 1,
        };
        let res: isize = (num + rot) % 4;
        match res {
            0 => MovDir::N,
            1 => MovDir::E,
            2 => MovDir::S,
            3 => MovDir::W,
            _ => panic!("WTF?!"),
        }
    }
}

enum Nav {
    M(MovDir, usize),
    R(RotDir),
    F(usize),
}

impl FromStr for Nav {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::new(InvalidData, "Invalid input line size"));
        }
        let mut chars = s.chars();
        let command = chars
            .next()
            .ok_or_else(|| Error::new(InvalidData, "Empty input line"))?;
        let number = chars.collect::<String>();
        let number = number
            .parse::<usize>()
            .or_else(|_| Err(Error::new(InvalidData, "Invalid input value")))?;
        match command {
            'N' => Ok(Nav::M(MovDir::N, number)),
            'S' => Ok(Nav::M(MovDir::S, number)),
            'E' => Ok(Nav::M(MovDir::E, number)),
            'W' => Ok(Nav::M(MovDir::W, number)),
            'F' => Ok(Nav::F(number)),
            'L' | 'R' => match (command, number) {
                ('L', 90) => Ok(Nav::R(RotDir::L)),
                ('R', 90) => Ok(Nav::R(RotDir::R)),
                (_, 180) => Ok(Nav::R(RotDir::F)),
                ('L', 270) => Ok(Nav::R(RotDir::R)),
                ('R', 270) => Ok(Nav::R(RotDir::L)),
                _ => Err(Error::new(InvalidData, "Invalid rotation value")),
            },
            _ => Err(Error::new(InvalidData, "Invalid input command")),
        }
    }
}

struct Gps {
    cmds: Vec<Nav>,
    facing: MovDir,
    pos_x: i128,
    pos_y: i128,
}

impl FromStr for Gps {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Gps {
            cmds: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
            facing: MovDir::E,
            pos_x: 0,
            pos_y: 0,
        })
    }
}

impl Gps {
    fn move_dir(pos: &mut (i128, i128), dir: MovDir, dist: usize) {
        match dir {
            MovDir::E => pos.0 += dist as i128,
            MovDir::W => pos.0 -= dist as i128,
            MovDir::N => pos.1 += dist as i128,
            MovDir::S => pos.1 -= dist as i128,
        }
    }

    fn run_all(&mut self) {
        //move out the values to avoid problems with borrowing
        let mut facing = self.facing;
        let mut pos = (self.pos_x, self.pos_y);

        for cmd in self.cmds.iter() {
            match cmd {
                Nav::R(dir) => facing = facing.rotate(*dir),
                Nav::M(dir, dist) => Gps::move_dir(&mut pos, *dir, *dist),
                Nav::F(dist) => Gps::move_dir(&mut pos, facing, *dist),
            }
        }

        self.facing = facing;
        self.pos_x = pos.0;
        self.pos_y = pos.1;
        //self.cmds.clear();
    }
}

fn solve1(input: &str) -> Result<usize, Box<Error>> {
    let mut gps: Gps = input.parse()?;
    gps.run_all();
    Ok((gps.pos_x.abs() + gps.pos_y.abs()) as usize)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    println!("P1: {}", solve1(&input)?);
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Box<Error>> {
    const INPUT: &str = "F10\nN3\nF7\nR90\nF11";
    assert_eq!(solve1(INPUT)?, 25);
    Ok(())
}

#[test]
fn test_rot() -> Result<(), Box<Error>> {
    const INPUT: &str = "R90\nL90\nL90\nR90\nR180\nL180\nL180\nL180\nR270\nL270";
    let mut gps: Gps = INPUT.parse()?;
    gps.run_all();
    assert_eq!((gps.pos_x, gps.pos_y), (0, 0));
    assert_eq!(gps.facing, MovDir::E);
    Ok(())
}

#[test]
fn test_mov() -> Result<(), Box<Error>> {
    const INPUT: &str = "F10\nN10\nS10\nE10\nW10";
    let mut gps: Gps = INPUT.parse()?;
    gps.run_all();
    assert_eq!((gps.pos_x, gps.pos_y), (10, 0));
    assert_eq!(gps.facing, MovDir::E);
    Ok(())
}

#[test]
fn test_mov_rot() -> Result<(), Box<Error>> {
    const INPUT: &str = "F10\nR90\nF10\nR90\nF10\nR90\nF10";
    let mut gps: Gps = INPUT.parse()?;
    gps.run_all();
    assert_eq!((gps.pos_x, gps.pos_y), (0, 0));
    assert_eq!(gps.facing, MovDir::N);
    Ok(())
}
