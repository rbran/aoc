//Oh Shit, this is just an idea that got out of hand, better not to think about

use std::cell::RefCell;
use std::env;
use std::error;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::rc::{Rc, Weak};

#[derive(Clone, Copy, Debug)]
enum Cmd {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
    End,
    Nil,
}

#[derive(Debug)]
struct ExecBranch {
    cmd: Cmd,
    index: Option<usize>, //position on the original file
    dst: Weak<RefCell<ExecBranch>>,
    xrefs: Vec<Weak<RefCell<ExecBranch>>>,
}

impl ExecBranch {
    fn parse(line: &str, index: Option<usize>) -> Result<ExecBranch, Box<dyn error::Error>> {
        let mut line = line.split(' ');
        let cmd = line
            .next()
            .ok_or_else(|| Box::new(Error::new(InvalidData, "")))?;
        let value = line
            .next()
            .ok_or_else(|| Box::new(Error::new(InvalidData, "")))?
            .parse::<isize>()?;
        let cmd = match cmd {
            "acc" => Cmd::Acc(value),
            "nop" => Cmd::Nop(value),
            "jmp" => Cmd::Jmp(value),
            _ => {
                return Err(Box::new(Error::new(InvalidData, "")));
            }
        };
        Ok(Self::new(cmd, index))
    }

    fn new(cmd: Cmd, index: Option<usize>) -> Self {
        ExecBranch {
            cmd,
            index,
            dst: Weak::new(),
            xrefs: vec![],
        }
    }
}
struct ExecTree {
    all: Vec<Rc<RefCell<ExecBranch>>>,
    end: Rc<RefCell<ExecBranch>>,     //detect invalid references
    invalid: Rc<RefCell<ExecBranch>>, //detect invalid references
}

impl ExecTree {
    fn new(data: &str) -> Result<Self, Box<dyn error::Error>> {
        //basic tree struct
        let ret = ExecTree {
            all: data
                .lines()
                .enumerate()
                .map(|(index, x)| {
                    match ExecBranch::parse(x, Some(index)) {
                        Ok(x) => Ok(Rc::new(RefCell::new(x))),
                        Err(x) => Err(x), //can we improve that?
                    }
                })
                .collect::<Result<Vec<_>, _>>()?, // ::<> TURBO FISH <>::
            end: Rc::new(RefCell::new(ExecBranch::new(Cmd::End, None))),
            invalid: Rc::new(RefCell::new(ExecBranch::new(Cmd::Nil, None))),
        };
        //assemble the tree by populating the dst and xrefs
        for index in 0..ret.all.len() {
            let branch = ret.all.get(index).unwrap();
            let (branch_cmd, branch_index) = {
                let branch = branch.borrow();
                (branch.cmd, branch.index.unwrap())
            };
            let dst_branch = match branch_cmd {
                Cmd::Jmp(branch_value) => {
                    let dst_index = branch_index as isize + branch_value;
                    if dst_index as usize == ret.all.len() {
                        &ret.end //valid end
                    } else {
                        ret.all.get(dst_index as usize).unwrap_or(&ret.invalid)
                    }
                }
                _ => ret.all.get(branch_index + 1).unwrap_or(&ret.end),
            };
            dst_branch.borrow_mut().xrefs.push(Rc::downgrade(branch));
            branch.borrow_mut().dst = Rc::downgrade(dst_branch);
        }
        Ok(ret)
    }
}

struct Cpu<'a> {
    root: &'a mut ExecTree,
    pc: Weak<RefCell<ExecBranch>>, //detect invalid references
    acc: isize,
    executed: Vec<usize>, //store all indexs already executed
}

impl<'a> Cpu<'a> {
    fn new(root: &'a mut ExecTree) -> Self {
        let pc = Rc::downgrade(root.all.get(0).unwrap());
        Cpu {
            root,
            pc,
            acc: 0,
            executed: vec![],
        }
    }

    fn solve_p1(&mut self) -> isize {
        let mut ret = 0;
        for i in self {
            ret = i; //that's ugly, improve that!!
        }
        ret
    }

    fn solve_p2(&mut self) -> Option<isize> {
        if self.executed.len() == 0 {
            self.solve_p1(); //if not executed, solve p1 first
        }
        //check if the default already is right
        if let Cmd::End = self.pc.upgrade()?.borrow().cmd {
            return Some(self.acc); //finished correctly
        }
        //backtrace the end, and find mark all possible endings:
        //list of instructions that result in a regular exit
        let mut possible_end = vec![];
        //the instructions that need processing
        let mut end_check = vec![];
        //quick start by adding all xrefs from end to the list to process
        end_check.extend(self.root.end.borrow().xrefs.iter().map(|x| Weak::clone(x)));
        loop {
            let check = match end_check.pop() {
                None => break, //no more instr to process
                Some(x) => x,
            };
            //insert instruction to the confirmed paths, if it didn't yet
            let check = check.upgrade()?;
            let index = check.borrow().index?;
            if !possible_end.contains(&index) {
                possible_end.push(index);
                //add all xrefs to the processing list
                end_check.extend(check.borrow().xrefs.iter().map(|x| Weak::clone(x)));
            }
        }

        //search for instruction that where executed and if modified, could
        //jmp to one of the possible_end instructions
        let mut found = None;
        for index in &self.executed {
            let exec = self.root.all.get(*index)?.borrow();
            let check_instr = match exec.cmd {
                Cmd::Jmp(_) => {
                    //if we transform this jmp in a nop
                    self.root.all.get(*index + 1)?.borrow()
                }
                Cmd::Nop(value) => {
                    //if the nop is converted to a jmp
                    let check_index = (*index as isize + value) as usize;
                    self.root.all.get(check_index)?.borrow()
                }
                _ => continue,
            };
            //if the next instr is the Cmd::End or one of possible_end, we found
            match check_instr.index {
                None => {
                    if let Cmd::End = check_instr.cmd {
                        found = Some(*index);
                        break;
                    }
                }
                Some(check_index) => {
                    if possible_end.contains(&check_index) {
                        found = Some(*index);
                        break;
                    }
                }
            }
        }
        //now we modify the instruction and execute the cpu again
        {
            let corrupt_ref = self.root.all.get(found?)?;
            let corrupt_cmd = corrupt_ref.borrow().cmd;
            let corrupt_index = corrupt_ref.borrow().index?;
            let (old_ref, new_ref) = match corrupt_cmd {
                Cmd::Jmp(value) => {
                    let mut corrupt = corrupt_ref.borrow_mut();
                    corrupt.cmd = Cmd::Nop(value);
                    (corrupt.dst.upgrade()?, self.root.all.get(found? + 1)?)
                }
                Cmd::Nop(value) => {
                    let mut corrupt = corrupt_ref.borrow_mut();
                    corrupt.cmd = Cmd::Jmp(value);
                    let new_index = (found? as isize + value) as usize;
                    (corrupt.dst.upgrade()?, self.root.all.get(new_index)?)
                }
                _ => panic!("WTF?"),
            };
            {
                let mut old = old_ref.borrow_mut();
                let old_dst_index = old
                    .xrefs
                    .iter()
                    .position(|x| x.upgrade().unwrap().borrow().index.unwrap() == corrupt_index);
                old.xrefs.remove(old_dst_index?);
                let mut new = new_ref.borrow_mut();
                new.xrefs.push(Rc::downgrade(corrupt_ref));
            }
            let mut corrupt = corrupt_ref.borrow_mut();
            corrupt.dst = Rc::downgrade(new_ref);
        }
        self.executed.clear();
        self.acc = 0;
        self.pc = Rc::downgrade(self.root.all.get(0)?);
        Some(self.solve_p1())
    }
}

impl<'a> Iterator for Cpu<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let instr = self.pc.upgrade()?;
        let instr = instr.borrow();
        let index = instr.index?;
        if self.executed.contains(&index) {
            return None; //if previously executed, stop, avoid cycles
        }
        self.executed.push(index);
        match instr.cmd {
            Cmd::Acc(value) => self.acc += value,
            Cmd::Nop(_) | Cmd::Jmp(_) => {}
            Cmd::End | Cmd::Nil => return None, //found a valid/invalid end
        }
        let dst = instr.dst.upgrade()?;
        self.pc = Rc::downgrade(&dst);
        Some(self.acc)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut tree = ExecTree::new(&input)?;
    let mut cpu = Cpu::new(&mut tree);
    println!("P1: {}", cpu.solve_p1());
    println!("P2: {:?}", cpu.solve_p2());
    Ok(())
}
