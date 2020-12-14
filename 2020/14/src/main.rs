use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

struct Program {
    bitmask: [Option<bool>; 36],
    commands: Vec<(usize, u64)>,
}

impl FromStr for Program {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Program {
            bitmask: [None; 36],
            commands: Vec::new(),
        };
        let mut lines = s.lines();
        let mask = lines.next().ok_or_else(|| Error::new(InvalidData, "bitmap1"))?;
        if mask.len() != 36 {
            return Err(Error::new(InvalidData, "bitmap2"));
        }
        for (i, bit) in mask.chars().enumerate() {
            match bit {
                '0' => ret.bitmask[36 - i - 1] = Some(false),
                '1' => ret.bitmask[36 - i - 1] = Some(true),
                'x' | 'X' => (),
                _ => return Err(Error::new(InvalidData, "bitmap3")),
            }
        }

        for command in lines {
            let mut values = command.split(&['[', ']', ' '][..]);
            let error = || Error::new(InvalidData, "command1");
            let mem = values.next().ok_or_else(error)?;
            let addr = values.next().ok_or_else(error)?;
            let empty = values.next().ok_or_else(error)?;
            let eq = values.next().ok_or_else(error)?;
            let value = values.next().ok_or_else(error)?;
            if mem != "mem" || eq != "=" || empty.len() != 0 {
                return Err(Error::new(InvalidData, "command2"));
            }
            let error = |_| Err(Error::new(InvalidData, "command3"));
            let addr = addr.parse::<usize>().or_else(error)?;
            let error = |_| Err(Error::new(InvalidData, "command4"));
            let value = value .parse::<u64>().or_else(error)?;
            ret.commands.push((addr, value));
        }

        Ok(ret)
    }
}

impl Program {
    fn apply_bitmask(&self, value: u64) -> u64 {
        let mut ret: u64 = value;
        for (i, bit) in self.bitmask.iter().enumerate() {
            match bit {
                None => (),
                Some(true) => ret |= 1u64 << i,
                Some(false) => ret &= !(1u64 << i),
            }
        }
        ret
    }
}

fn parse_programs(s: &str) -> Result<Vec<Program>, Error> {
    let programs = s.split("mask = ");
    let mut ret = Vec::new();
    for program in programs {
        if program.len() == 0 {
            continue;
        }
        ret.push(program.parse()?);
    }
    Ok(ret)
}

fn solve1(input: &str) -> Result<usize, Box<Error>> {
    let programs = parse_programs(input)?;
    let mut mem = HashMap::new();

    for program in programs {
        for (addr, value) in &program.commands {
            mem.insert(*addr, program.apply_bitmask(*value));
        }
    }
    Ok(mem.values().sum::<u64>() as usize)
}

fn solve2(input: &str) -> Result<usize, Box<Error>> {
    unimplemented!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    println!("P1: {}", solve1(&input)?);
    println!("P1: {}", solve2(&input)?);
    Ok(())
}

#[test]
fn test_part_1_1() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                         mem[8] = 11\nmem[7] = 101\nmem[8] = 0";
    assert_eq!(solve1(INPUT)?, 165);
    Ok(())
}

#[test]
fn test_part_1_2() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n\
                         mem[8] = 11\nmem[7] = 101\nmem[8] = 0";
    assert_eq!(solve1(INPUT)?, 101);
    Ok(())
}

#[test]
fn test_part_1_3() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n\
                         mem[8] = 11\nmem[7] = 101\nmem[8] = 0\n\
                         mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n\
                         mem[1] = 11\nmem[2] = 11\nmem[3] = 11";
    assert_eq!(solve1(INPUT)?, 101 + 11 * 3);
    Ok(())
}

#[test]
fn test_bitmask_1() -> Result<(), Box<dyn std::error::Error>> {
    const INPUT: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX0\n\
                         mem[8] = 11\nmem[7] = 101\nmem[8] = 0";
    let program = INPUT.parse::<Program>()?;
    assert_eq!(program.apply_bitmask(0), 0);
    assert_eq!(program.apply_bitmask(1), 0);
    assert_eq!(program.apply_bitmask(2), 2);
    assert_eq!(program.apply_bitmask(3), 2);
    assert_eq!(program.apply_bitmask(255), 254);
    assert_eq!(program.apply_bitmask(0b101010101), 0b101010100);
    assert_eq!(program.apply_bitmask(0b10101010), 0b10101010);
    Ok(())
}
