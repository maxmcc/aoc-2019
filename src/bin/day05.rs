#![feature(slice_patterns)]

use aoc::intcode::*;

fn main() {
    let input = include_str!("../../input/day05.in");
    let program = input.parse::<Program>().unwrap();
    part1(&program);
    part2(&program);
}

fn part1(program: &Program) {
    let output = run_machine(&program, 1.into());
    println!("{}", output.0);
}

fn part2(program: &Program) {
    let output = run_machine(&program, 5.into());
    println!("{}", output.0);
}

fn run_machine(program: &Program, input: mem::Value) -> mem::Value {
    let mut machine = Machine::default_io(&program);
    machine.input.queue.push_back(input);
    machine.run();
    *machine.output.buffer.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_known_answers() {
        let input = include_str!("../../input/day05.in");
        let program = input.parse::<Program>().unwrap();
        assert_eq!(run_machine(&program, 1.into()), 16348437.into());
        assert_eq!(run_machine(&program, 5.into()), 6959377.into());
    }
}
