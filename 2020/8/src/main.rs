use std::env;
use std::error;
use std::fs;
use std::io::{Error, ErrorKind};

enum Cmd {
    Acc,
    Jmp,
    Nop,
}

struct Exec {
    cmd: Cmd,
    value: isize,
    exec: bool,
}

fn parse_instruction(line: &str) -> Result<Exec, Box<dyn error::Error>> {
    let mut line = line.split(' ');
    let cmd = line
        .next()
        .ok_or_else(|| Box::new(Error::new(ErrorKind::InvalidData, "")))?;
    let value = line
        .next()
        .ok_or_else(|| Box::new(Error::new(ErrorKind::InvalidData, "")))?;
    let cmd = match cmd {
        "acc" => Cmd::Acc,
        "nop" => Cmd::Nop,
        "jmp" => Cmd::Jmp,
        _ => {
            return Err(Box::new(Error::new(ErrorKind::InvalidData, "")));
        }
    };
    let value = value.parse::<isize>()?;
    Ok(Exec {
        cmd,
        value,
        exec: false,
    })
}

struct Cpu {
    cmds: Vec<Exec>,
    pc: usize,
    acc: isize,
}

impl Iterator for Cpu {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let cmd = self.cmds.get_mut(self.pc)?;
        if cmd.exec {
            return None;
        }
        cmd.exec = true;
        match cmd.cmd {
            Cmd::Nop => {
                self.pc += 1;
                Some(self.acc)
            },
            Cmd::Acc => {
                self.acc += cmd.value;
                self.pc += 1;
                Some(self.acc)
            }
            Cmd::Jmp => {
                if self.pc > isize::MAX as usize {
                    return None;
                }
                let pc_res = self.pc as isize + cmd.value;
                if pc_res < 0 {
                    return None;
                }
                let pc_res = pc_res as usize;
                if pc_res >= self.cmds.len() {
                    return None;
                }
                self.pc = pc_res as usize;
                Some(self.acc)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // assuming all passwords are lower case
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let cmds: Result<Vec<Exec>, _> = input.lines().map(parse_instruction).collect();
    let cmds = cmds?;
    let sim1 = Cpu {cmds, acc:0, pc: 0};
    println!("P1: {}", sim1.last().unwrap_or(0));
    Ok(())
}
