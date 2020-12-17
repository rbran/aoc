//My solution for part 1 was so good, that I could solve part 2 by just coping
//some code.
//Although I could implement a N-dimention solution, in compile and runtime;
//I'll leave that for the future me.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

struct Input {
    data: Vec<Vec<bool>>,
}

impl FromStr for Input {
    type Err = Box<Error>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = Input {
            data: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|cube| match cube {
                            '#' => Ok(true),
                            '.' => Ok(false),
                            _ => Err(Box::new(Error::new(
                                InvalidData,
                                "Invalid Input Char",
                            ))),
                        })
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        };
        //some checks: lines > 0 and columns > 0 and all lines len are equal
        if input.data.len() == 0 {
            return Err(Box::new(Error::new(InvalidData, "Input is empty")));
        }
        let lines_len = input.data.first().unwrap().len();
        if lines_len == 0 {
            return Err(Box::new(Error::new(InvalidData, "Line is empty")));
        }
        //Input is always square?
        //if input.data.len() != lines_len {
        //    return Err(Box::new(Error::new(
        //        InvalidData,
        //        "Input is not square",
        //    )));
        //}
        for line in &input.data {
            if line.len() != lines_len {
                return Err(Box::new(Error::new(
                    InvalidData,
                    "Lines len differ",
                )));
            }
        }
        Ok(input)
    }
}

const fn gen_neighbours() -> [(isize, isize, isize); 26] {
    let mut ret = [(0, 0, 0); 26];
    let mut i = 0;
    let mut pos = 0;
    while i < 27 {
        let mut acc = i as isize;
        let x = acc % 3;
        acc /= 3;
        let y = acc % 3;
        acc /= 3;
        let z = acc % 3;
        if x == 1 && y == 1 && z == 1 {
            i += 1;
            continue;
        }
        ret[pos] = (x - 1, y - 1, z - 1);
        pos += 1;
        i += 1;
    }
    ret
}

const NEIGHBOURS: [(isize, isize, isize); 26] = gen_neighbours();

const fn gen_neighbours_2() -> [(isize, isize, isize, isize); 80] {
    let mut ret = [(0, 0, 0, 0); 80];
    let mut i = 0;
    let mut pos = 0;
    while i < 81 {
        let mut acc = i as isize;
        let x = acc % 3;
        acc /= 3;
        let y = acc % 3;
        acc /= 3;
        let z = acc % 3;
        acc /= 3;
        let w = acc % 3;
        if x == 1 && y == 1 && z == 1 && w == 1 {
            i += 1;
            continue;
        }
        ret[pos] = (x - 1, y - 1, z - 1, w - 1);
        pos += 1;
        i += 1;
    }
    ret
}

const NEIGHBOURS_2: [(isize, isize, isize, isize); 80] = gen_neighbours_2();


const fn sum_pos(
    p1: &(isize, isize, isize),
    p2: &(isize, isize, isize),
) -> (isize, isize, isize) {
    (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2)
}

const fn sum_pos_2(
    p1: &(isize, isize, isize, isize),
    p2: &(isize, isize, isize, isize),
) -> (isize, isize, isize, isize) {
    (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2, p1.3 + p2.3)
}


fn print_box(pocket: &HashMap<(isize, isize, isize), bool>) {
    if pocket.len() == 0 {
        return;
    }
    let (mut max_x, mut min_x, mut max_y, mut min_y, mut max_z, mut min_z) =
        (isize::MIN, isize::MAX, isize::MIN, isize::MAX, isize::MIN, isize::MAX);
    for &(cube_x, cube_y, cube_z) in pocket.keys() {
        if max_x < cube_x {
            max_x = cube_x;
        }
        if min_x > cube_x {
            min_x = cube_x;
        }
        if max_y < cube_y {
            max_y = cube_y;
        }
        if min_y > cube_y {
            min_y = cube_y;
        }
        if max_z < cube_z {
            max_z = cube_z;
        }
        if min_z > cube_z {
            min_z = cube_z;
        }
    }
    for z in min_z..=max_z {
        println!("z = {}", z);
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if pocket.contains_key(&(x, y, z)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn print_box_2(pocket: &HashMap<(isize, isize, isize, isize), bool>) {
    if pocket.len() == 0 {
        return;
    }
    let (mut max_x, mut min_x, mut max_y, mut min_y, mut max_z, mut min_z, mut min_w, mut max_w) =
        (isize::MIN, isize::MAX, isize::MIN, isize::MAX, isize::MIN, isize::MAX, isize::MAX, isize::MIN);
    for &(cube_x, cube_y, cube_z, cube_w) in pocket.keys() {
        if max_x < cube_x {
            max_x = cube_x;
        }
        if min_x > cube_x {
            min_x = cube_x;
        }
        if max_y < cube_y {
            max_y = cube_y;
        }
        if min_y > cube_y {
            min_y = cube_y;
        }
        if max_z < cube_z {
            max_z = cube_z;
        }
        if min_z > cube_z {
            min_z = cube_z;
        }
        if max_w < cube_w {
            max_w = cube_w;
        }
        if min_w > cube_w {
            min_w = cube_w;
        }
    }
    for w in min_w..=max_w {
    for z in min_z..=max_z {
        println!("z = {}, w = {}", z, w);
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if pocket.contains_key(&(x, y, z, w)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    }
}

fn solve1(input: &Input) -> Result<usize, Box<Error>> {
    //the pocket dimentions is a HashMap, the key are the coordinates,
    //only active cubes have entry, inactive don't have entries,
    //Value is used to avoid duplicated processing between cycles:
    //false is unprocessed cube, true is a processed cube.
    let mut pocket: HashMap<(isize, isize, isize), bool> = HashMap::new();
    let len_y = input.data.len();
    let len_x = input.data[0].len();
    let middle_x = len_x as isize / 2;
    let middle_y = len_y as isize / 2;
    for (y, line) in input.data.iter().enumerate() {
        for (x, cube) in line.iter().enumerate() {
            if *cube {
                let y = (len_y - y - 1) as isize - middle_y; //y is mirrored
                let x = x as isize - middle_x;
                pocket.insert((x, y, 0), false);
            }
        }
    }
    //println!("Before any cycles:\n");
    //print_box(&pocket);

    //cycle 6 times
    for _cycle in 0..6 {
        let mut new_pocket = HashMap::new();
        let pocket_tmp = pocket.clone(); //TODO: improve this
        for pos_check in pocket_tmp.keys() {
            //for each active cubes, check all the neighbour cubes and itself
            for nei_pos in NEIGHBOURS.iter().chain([(0, 0, 0)].iter()) {
                let pos_cube = sum_pos(pos_check, nei_pos);
                let cube = pocket.entry(pos_cube);
                let cube_active = match cube {
                    Entry::Vacant(_) => false,
                    Entry::Occupied(_) => true,
                };
                let cube_processed = cube.or_insert(false);
                if *cube_processed {
                    //cube was already verified, next
                    continue;
                }
                *cube_processed = true;
                //check the number of active boxes around it
                let mut active = 0;
                for nei_pos in NEIGHBOURS.iter() {
                    let pos_count = sum_pos(&pos_cube, nei_pos);
                    let has_key = pocket_tmp.contains_key(&pos_count);
                    active += has_key as usize;
                }
                if cube_active {
                    //If a cube is active and exactly 2 or 3 of its neighbors
                    //are also active, the cube remains active. Otherwise,
                    //the cube becomes inactive.
                    if active == 2 || active == 3 {
                        new_pocket.insert(pos_cube, false);
                    }
                } else {
                    //If a cube is inactive but exactly 3 of its neighbors are
                    //active, the cube becomes active. Otherwise, the cube
                    //remains inactive.
                    if active == 3 {
                        new_pocket.insert(pos_cube, false);
                    }
                }
            }
        }
        pocket = new_pocket;
        //println!("\n\nAfter {} cycle:\n", _cycle + 1);
        //print_box(&pocket);
    }

    Ok(pocket.len())
}

fn solve2(input: &Input) -> Result<usize, Box<Error>> {
    //the pocket dimentions is a HashMap, the key are the coordinates,
    //only active cubes have entry, inactive don't have entries,
    //Value is used to avoid duplicated processing between cycles:
    //false is unprocessed cube, true is a processed cube.
    let mut pocket: HashMap<(isize, isize, isize, isize), bool> = HashMap::new();
    let len_y = input.data.len();
    let len_x = input.data[0].len();
    let middle_x = len_x as isize / 2;
    let middle_y = len_y as isize / 2;
    for (y, line) in input.data.iter().enumerate() {
        for (x, cube) in line.iter().enumerate() {
            if *cube {
                let y = (len_y - y - 1) as isize - middle_y; //y is mirrored
                let x = x as isize - middle_x;
                pocket.insert((x, y, 0, 0), false);
            }
        }
    }
    //println!("Before any cycles:\n");
    //print_box_2(&pocket);

    //cycle 6 times
    for _cycle in 0..6 {
        let mut new_pocket = HashMap::new();
        let pocket_tmp = pocket.clone(); //TODO: improve this
        for pos_check in pocket_tmp.keys() {
            //for each active cubes, check all the neighbour cubes and itself
            for nei_pos in NEIGHBOURS_2.iter().chain([(0, 0, 0, 0)].iter()) {
                let pos_cube = sum_pos_2(pos_check, nei_pos);
                let cube = pocket.entry(pos_cube);
                let cube_active = match cube {
                    Entry::Vacant(_) => false,
                    Entry::Occupied(_) => true,
                };
                let cube_processed = cube.or_insert(false);
                if *cube_processed {
                    //cube was already verified, next
                    continue;
                }
                *cube_processed = true;
                //check the number of active boxes around it
                let mut active = 0;
                for nei_pos in NEIGHBOURS_2.iter() {
                    let pos_count = sum_pos_2(&pos_cube, nei_pos);
                    let has_key = pocket_tmp.contains_key(&pos_count);
                    active += has_key as usize;
                }
                if cube_active {
                    //If a cube is active and exactly 2 or 3 of its neighbors
                    //are also active, the cube remains active. Otherwise,
                    //the cube becomes inactive.
                    if active == 2 || active == 3 {
                        new_pocket.insert(pos_cube, false);
                    }
                } else {
                    //If a cube is inactive but exactly 3 of its neighbors are
                    //active, the cube becomes active. Otherwise, the cube
                    //remains inactive.
                    if active == 3 {
                        new_pocket.insert(pos_cube, false);
                    }
                }
            }
        }
        pocket = new_pocket;
        //println!("\n\nAfter {} cycle:\n", _cycle + 1);
        //print_box_2(&pocket);
    }

    Ok(pocket.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    println!("P1: {}", solve1(&input)?);
    println!("P2: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_part1() -> Result<(), Box<Error>> {
    const INPUT: &str = ".#.\n..#\n###";
    let input = INPUT.parse()?;
    assert_eq!(solve1(&input)?, 112);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), Box<Error>> {
    const INPUT: &str = ".#.\n..#\n###";
    let input = INPUT.parse()?;
    assert_eq!(solve2(&input)?, 848);
    Ok(())
}
