#![feature(slice_patterns)]

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
    let output = computer.run(vec![1]);
    println!("{:?}", output);
}

fn part2(input: &str) {
    let mut computer = Computer::from_str(input);
    let output = computer.run(vec![5]);
    println!("{:?}", output);
}
