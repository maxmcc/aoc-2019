#![feature(slice_patterns)]

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut computer = Computer::from_str(input);
    computer.set_noun(12);
    computer.set_verb(2);
    computer.run();
    println!("{}", computer.run());
}

fn part2(input: &str) {
    let computer = Computer::from_str(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = computer.clone();
            computer.set_noun(noun);
            computer.set_verb(verb);
            if computer.run() == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("couldn't find noun & verb");
}

#[derive(Clone, Debug)]
struct Computer {
    memory: Vec<usize>,
    instruction_ptr: usize,
}

impl Computer {
    fn from_str(string: &str) -> Self {
        let memory: Vec<_> = string.split(',').map(|x| x.parse().unwrap()).collect();
        Computer::new(&memory)
    }

    fn new(memory: &[usize]) -> Self {
        Computer {
            memory: memory.to_vec(),
            instruction_ptr: 0,
        }
    }

    fn set_noun(&mut self, noun: usize) {
        self.memory[1] = noun;
    }

    fn set_verb(&mut self, verb: usize) {
        self.memory[2] = verb;
    }

    fn binary_op<F>(&mut self, lhs: usize, rhs: usize, dst: usize, op: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        self.memory[dst] = op(self.memory[lhs], self.memory[rhs]);
        self.instruction_ptr += 4;
    }

    fn run(&mut self) -> usize {
        loop {
            match self.memory[self.instruction_ptr..] {
                [1, lhs, rhs, dst, ..] => self.binary_op(lhs, rhs, dst, |x, y| x + y),
                [2, lhs, rhs, dst, ..] => self.binary_op(lhs, rhs, dst, |x, y| x * y),
                [99] | [99, ..] => break,
                _ => panic!("unrecognized opcode"),
            }
        }
        self.memory[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_computer() {
        let mut computer = Computer::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        computer.run();
        assert_eq!(
            computer.memory,
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        let mut computer = Computer::new(&[1, 0, 0, 0, 99]);
        computer.run();
        assert_eq!(computer.memory, &[2, 0, 0, 0, 99]);

        let mut computer = Computer::new(&[2, 3, 0, 3, 99]);
        computer.run();
        assert_eq!(computer.memory, &[2, 3, 0, 6, 99]);

        let mut computer = Computer::new(&[2, 4, 4, 5, 99, 0]);
        computer.run();
        assert_eq!(computer.memory, &[2, 4, 4, 5, 99, 9801]);

        let mut computer = Computer::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run();
        assert_eq!(computer.memory, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
