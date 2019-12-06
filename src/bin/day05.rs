#![feature(slice_patterns)]

use aoc::intcode::Computer;

fn main() {
    let input = include_str!("../../input/day05.in");
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
