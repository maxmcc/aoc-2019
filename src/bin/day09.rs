use aoc::intcode::*;

fn main() {
    let input = include_str!("../../input/day09.in");
    let program = input.parse::<Program>().unwrap();
    let mut machine = Machine::default_io(&program);
    machine.input.queue.push_back(2.into());
    let status = machine.run();
    assert_eq!(status, vm::Status::Halted);
    let out = dbg!(machine.output.buffer);
    assert_eq!(out.len(), 1);
    println!("{}", out[0].0);
}
