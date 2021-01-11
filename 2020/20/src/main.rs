use std::collections::HashMap;
use std::convert::{From, Into};
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

//I'll assume the input is always 10x10, 100bits
struct Input {
    tiles: HashMap<usize, u128>,
}

fn error(s: &str) -> Err {
    Box::new(Error::new(InvalidData, s.to_string()))
}

impl FromStr for Input {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const DEF_ERR: &str = "Invalid Input";
        let tiles_str = s.split("\n\n");
        let mut tiles = HashMap::new();
        for tile in tiles_str {
            if tile.len() == 0 {
                continue;
            }
            let mut lines = tile.lines();
            let title = lines.next().ok_or_else(|| error("Empty input"))?;
            let mut title = title.split(|x| x == ' ' || x == ':');
            if title.next().ok_or_else(|| error(DEF_ERR))? != "Tile" {
                return Err(error("Invalid title format"));
            }
            let index = title.next().ok_or_else(|| error(DEF_ERR))?;
            let index = index.parse::<usize>()?;
            let tile_pixels = lines
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, pixel)| match pixel {
                            '#' => Ok(1u128 << ((9 - x) + (10 * (9 - y)))),
                            '.' => Ok(0u128),
                            _ => Err(error("Invalid pixel format")),
                        })
                        .sum::<Result<u128, _>>()
                })
                .sum::<Result<_, _>>()?;
            tiles.insert(index, tile_pixels);
        }
        Ok(Input { tiles })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Ori {
    Up,
    Right,
    Down,
    Left,
}

impl From<isize> for Ori {
    fn from(v: isize) -> Self {
        match v.abs() % 4 {
            0 => Ori::Up,
            1 => Ori::Right,
            2 => Ori::Down,
            3 => Ori::Left,
            _ => panic!("WTF!?!?!?"),
        }
    }
}

impl Into<isize> for Ori {
    fn into(self) -> isize {
        match self {
            Ori::Up => 0,
            Ori::Right => 1,
            Ori::Down => 2,
            Ori::Left => 3,
        }
    }
}

impl Ori {
    fn is_h(&self) -> bool {
        match *self {
            Ori::Left | Ori::Right => true,
            _ => false,
        }
    }

    fn opose(&self) -> Self {
        match *self {
            Ori::Up => Ori::Down,
            Ori::Right => Ori::Left,
            Ori::Down => Ori::Up,
            Ori::Left => Ori::Right,
        }
    }
}

struct Tile {
    pixels: u128,
    index: usize,
    ori: Ori,
    flip: bool, //invert the x axis
}

const fn flip_border(mut n: u16) -> u16 {
    let mut acc = 0;
    let mut i = 0;
    while i < 10 {
        acc <<= 1;
        acc |= n & 1;
        n >>= 1;
        i += 1;
    }
    acc
}

impl Tile {
    fn get_abs_side(&self, side: Ori) -> Ori {
        let mut abs_side = Ori::from(4 + side as isize - self.ori as isize);
        if self.flip && abs_side.is_h() {
            abs_side = abs_side.opose();
        }
        abs_side
    }
    //the number is calculated going clock-wise
    fn get_border(&self, side: Ori) -> u16 {
        let abs_side = self.get_abs_side(side);
        match abs_side {
            Ori::Down => {
                let value = (self.pixels & 0x3ff) as u16;
                if self.flip {
                    value
                } else {
                    flip_border(value)
                }
            }
            Ori::Up => {
                let value = ((self.pixels & (0x3ff << 90)) >> 90) as u16;
                if self.flip {
                    flip_border(value)
                } else {
                    value
                }
            }
            Ori::Right => {
                let mut value = 0;
                let mut acc = self.pixels;
                for _ in 0..10 {
                    value <<= 1;
                    value |= (acc & 1) as u16;
                    acc >>= 10;
                }
                if self.flip {
                    value
                } else {
                    flip_border(value)
                }
            }
            Ori::Left => {
                let mut value = 0;
                let mut acc = self.pixels >> 9;
                for _ in 0..10 {
                    value <<= 1;
                    value |= (acc & 1) as u16;
                    acc >>= 10;
                }
                if self.flip {
                    flip_border(value)
                } else {
                    value
                }
            }
        }
    }

    // modify b to connect to self, return the side or self that 'b' connect
    fn connect(&self, b: &mut Tile) -> Option<Ori> {
        //discard b ori and flip, it will be what we make him be
        b.ori = Ori::Up;
        b.flip = false;
        //search all sides of self
        for side_self in 0..4 {
            let ori = Ori::from(side_self);
            let border_f = self.get_border(ori);
            let border_n = flip_border(border_f); //the borders connect mirrored
                                                  //against all sides of b
            for side_b in 0..4 {
                let side_b = Ori::from(side_b);
                let border_b = b.get_border(side_b);
                if border_b == border_n {
                    //side_b need to be the new side_self oposite, so it fit
                    b.ori =
                        Ori::from(ori.opose() as isize + 4 - side_b as isize);
                    return Some(ori);
                } else if border_b == border_f {
                    b.ori =
                        Ori::from(ori.opose() as isize + 4 - side_b as isize);
                    if side_b.is_h() {
                        b.ori = b.ori.opose();
                    }
                    b.flip = !b.flip;
                    return Some(ori);
                }
            }
        }
        None
    }
}

#[test]
fn test_get_boder() -> Result<(), Err> {
    const INPUT: &str = "Tile 1:
..........
##########
.#.#.#.#.#
#.#.#.#.#.
##########
##########
..........
..........
##..##..##
..##..##..";
    let input: Input = INPUT.parse()?;
    let mut tile = Tile {
        pixels: *input.tiles.get(&1).ok_or_else(|| error(""))?,
        index: 1,
        ori: Ori::Up,
        flip: false,
    };
    assert_eq!(tile.get_border(Ori::Up), 0b0000000000);
    assert_eq!(tile.get_border(Ori::Left), 0b0100111010);
    assert_eq!(tile.get_border(Ori::Down), 0b0011001100);
    assert_eq!(tile.get_border(Ori::Right), 0b0110110010);
    tile.flip = true;
    tile.ori = Ori::Left;
    assert_eq!(tile.get_border(Ori::Up), 0b0101110010);
    assert_eq!(tile.get_border(Ori::Left), 0b0000000000);
    assert_eq!(tile.get_border(Ori::Down), 0b0100110110);
    assert_eq!(tile.get_border(Ori::Right), 0b0011001100);
    Ok(())
}

#[test]
fn test_connect() -> Result<(), Err> {
    const INPUT: &str = "Tile 1:
..........
.........#
..........
.........#
..........
..........
..........
..........
..........
..........

Tile 2:
.#.#......
##########
##########
##########
##########
##########
##########
##########
##########
##########

Tile 3:
#########.
#########.
#########.
#########.
#########.
#########.
##########
#########.
##########
#########.

Tile 4:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 5:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.
";
    let input: Input = INPUT.parse()?;
    let tile1 = Tile {
        pixels: *input.tiles.get(&1).ok_or_else(|| error(""))?,
        index: 1,
        ori: Ori::Up,
        flip: false,
    };
    let mut tile2 = Tile {
        pixels: *input.tiles.get(&2).ok_or_else(|| error(""))?,
        index: 2,
        ori: Ori::Up,
        flip: false,
    };
    let mut tile3 = Tile {
        pixels: *input.tiles.get(&3).ok_or_else(|| error(""))?,
        index: 3,
        ori: Ori::Up,
        flip: false,
    };
    let tile4 = Tile {
        pixels: *input.tiles.get(&4).ok_or_else(|| error(""))?,
        index: 4,
        ori: Ori::Left,
        flip: false,
    };
    let mut tile5 = Tile {
        pixels: *input.tiles.get(&5).ok_or_else(|| error(""))?,
        index: 5,
        ori: Ori::Up,
        flip: false,
    };
    let res2 = tile1.connect(&mut tile2);
    let res3 = tile1.connect(&mut tile3);
    let res4 = tile4.connect(&mut tile5);
    assert_eq!(res2, Some(Ori::Right));
    assert_eq!(tile2.ori, Ori::Left);
    assert!(tile2.flip);
    assert_eq!(res3, Some(Ori::Right));
    assert_eq!(tile3.ori, Ori::Down);
    assert!(!tile3.flip);
    assert_eq!(res4, Some(Ori::Right));
    assert_eq!(tile5.ori, Ori::Down);
    assert!(!tile5.flip);
    Ok(())
}

fn mount_image(
    input: &Input,
) -> Result<(usize, HashMap<(usize, usize), Tile>), Err> {
    //I'll try assuming only one connection is possible
    //I'll generate a struct Tile
    let mut tiles_iter = input.tiles.iter();
    let (first_tile_index, first_tile) = tiles_iter.next().unwrap();
    let first_tile = Tile {
        pixels: *first_tile,
        index: *first_tile_index,
        ori: Ori::Up,
        flip: false,
    };
    let mut tiles_available: Vec<_> = tiles_iter
        .map(|(index, tile)| Tile {
            pixels: *tile,
            index: *index,
            ori: Ori::Up,
            flip: false,
        })
        .collect();

    //I'll use a grid in hashmap, starting with a random tile
    let mut grid: HashMap<(isize, isize), Tile> = HashMap::new();
    grid.insert((0, 0), first_tile);
    //keep adding tiles to the grid, until there is not more tiles available
    loop {
        if tiles_available.len() == 0 {
            break;
        }

        let mut found = false;
        let mut index = 0;
        let mut new_pos = (0, 0);
        //try all the available tiles, at least one should fit on the grid
        for (try_index, mut try_tile) in tiles_available.iter_mut().enumerate()
        {
            //try on all the grid possitions
            for (grid_pos, grid_tile) in grid.iter() {
                if let Some(ori) = grid_tile.connect(&mut try_tile) {
                    found = true;
                    index = try_index;
                    //insert to the grid
                    new_pos = match ori {
                        Ori::Up => (grid_pos.0, grid_pos.1 + 1),
                        Ori::Down => (grid_pos.0, grid_pos.1 - 1),
                        Ori::Right => (grid_pos.0 + 1, grid_pos.1),
                        Ori::Left => (grid_pos.0 - 1, grid_pos.1),
                    };
                    break;
                }
            }
            if found {
                break;
            }
        }

        if found {
            grid.insert(new_pos, tiles_available.remove(index));
        } else {
            println!(
                "grid len {}, available {}",
                grid.len(),
                tiles_available.len()
            );
            panic!("");
        }
    }

    //TODO: check is a square
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    for (pos_x, pos_y) in grid.keys() {
        if min_x > *pos_x {
            min_x = *pos_x;
        }
        if max_x < *pos_x {
            max_x = *pos_x;
        }
        if min_y > *pos_y {
            min_y = *pos_y;
        }
        if max_y < *pos_y {
            max_y = *pos_y;
        }
    }
    let len_x = max_x - min_x;
    let len_y = max_y - min_y;
    if len_x != len_y {
        return Err(error("invalid grid size"));
    }
    //remap the hasmap so the grid goes 0,0 up and never negative
    let mut ret = HashMap::new();
    for (k, v) in grid.drain() {
        let new_key = ((k.0 - min_x) as usize, (k.1 - min_y) as usize);
        ret.insert(new_key, v);
    }
    Ok((len_x as usize, ret))
}

fn solve1(input: &Input) -> Result<usize, Err> {
    let (size, grid) = mount_image(input)?;
    let mut ret = 1;
    for pos in [(0, 0), (0, size), (size, 0), (size, size)].iter() {
        ret *= grid.get(pos).unwrap().index;
    }
    Ok(ret)
}

fn solve2(input: &Input) -> Result<usize, Err> {
    unimplemented!();
}

fn main() -> Result<(), Err> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    println!("P1: {}", solve1(&input)?);
    println!("P2: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_1() -> Result<(), Err> {
    const INPUT: &str = "Tile 2311:
..........
##########
.#.#.#.#.#
#.#.#.#.#.
##########
##########
..........
..........
##..##..##
..##..##..";
    const RESS: u128 = 1237134482653816829944581324;
    let input: Input = INPUT.parse()?;
    assert_eq!(input.tiles.len(), 1);
    let tile = input.tiles.get(&2311).ok_or_else(|| error(""))?;
    assert_eq!(*tile, RESS);
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Err> {
    const INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    let input = INPUT.parse()?;
    assert_eq!(solve1(&input)?, 20899048083289);
    Ok(())
}
