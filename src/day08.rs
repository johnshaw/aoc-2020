use failure::Error;
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
enum Op {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_code(data: &str) -> Vec<Op> {
    data.lines()
        .map(|l| {
            let (o, p) = l.split_whitespace().next_tuple().unwrap();
            let p = p.parse().unwrap();
            match o {
                "acc" => Op::Acc(p),
                "jmp" => Op::Jmp(p),
                "nop" => Op::Nop(p),
                _ => panic!("Invalid program"),
            }
        })
        .collect()
}

#[derive(Clone)]
struct Machine {
    code: Vec<Op>,
    pc: i64,
    acc: i64,
}

#[derive(PartialEq)]
enum ExitCode {
    Success,
    BadPc,
    InfLoop,
}

impl Machine {
    fn new(source: &str) -> Machine {
        Machine {
            code: parse_code(source),
            pc: 0,
            acc: 0,
        }
    }

    fn get_acc(&self) -> i64 {
        self.acc
    }

    fn get_pc(&self) -> i64 {
        self.pc
    }

    fn step(&mut self) -> Option<ExitCode> {
        if self.pc == self.code.len().try_into().unwrap() {
            return Some(ExitCode::Success);
        }

        if self.pc < 0 || self.pc >= self.code.len().try_into().unwrap() {
            return Some(ExitCode::BadPc);
        }

        let op = self.code[self.pc as usize];

        match op {
            Op::Acc(p) => {
                self.acc += p;
                self.pc += 1;
            }
            Op::Jmp(p) => self.pc += p,
            Op::Nop(_) => self.pc += 1,
        }

        None
    }

    fn run(&mut self) -> ExitCode {
        let mut seen = HashSet::new();
        loop {
            if seen.contains(&self.get_pc()) {
                return ExitCode::InfLoop;
            }

            seen.insert(self.get_pc());

            if let Some(c) = self.step() {
                return c;
            }
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
    }
}

pub fn part_a(data: String) -> Result<i64, Error> {
    let mut m = Machine::new(&data);
    m.run();
    Ok(m.get_acc())
}

pub fn part_b(data: String) -> Result<i64, Error> {
    let mut m = Machine::new(&data);
    let pos: Vec<_> = m
        .code
        .iter()
        .positions(|o| match o {
            Op::Jmp(_) | Op::Nop(_) => true,
            _ => false,
        })
        .collect();

    fn swap(o: Op) -> Op {
        match o {
            Op::Jmp(p) => Op::Nop(p),
            Op::Nop(p) => Op::Jmp(p),
            x => x,
        }
    }

    for idx in pos.into_iter() {
        m.reset();
        m.code[idx] = swap(m.code[idx]);
        if m.run() == ExitCode::Success {
            return Ok(m.get_acc());
        }
        m.code[idx] = swap(m.code[idx]);
    }

    Ok(0)
}
