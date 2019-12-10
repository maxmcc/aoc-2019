use aoc::intcode::*;

fn main() {
    let input = include_str!("../../input/day09.in");
    let program = input.parse::<Program>().unwrap();
    part1(&program);
    part2(&program);
}

fn part1(program: &Program) {
    let value = run_with_input(&program, 1.into());
    println!("{}", value.0);
}

fn part2(program: &Program) {
    let value = run_with_input(&program, 2.into());
    println!("{}", value.0);
}

fn run_with_input(program: &Program, input: mem::Value) -> mem::Value {
    let mut machine = Machine::default_io(&program);
    machine.input.queue.push_back(input);
    let status = machine.run();
    assert_eq!(status, vm::Status::Halted);
    *machine.output.buffer.first().unwrap()
}
