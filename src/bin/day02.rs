use aoc::intcode::Computer;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut computer = Computer::from_str(input);
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    computer.run_no_io();
    println!("{}", computer.memory[0]);
}

fn part2(input: &str) {
    let computer = Computer::from_str(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = computer.clone();
            computer.memory[1] = noun;
            computer.memory[2] = verb;
            computer.run_no_io();
            if computer.memory[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("couldn't find noun & verb");
}
