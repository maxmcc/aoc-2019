use aoc::intcode::*;

fn main() {
    let input = include_str!("../../input/day02.in");
    let program = input.parse::<Program>().unwrap();
    part1(&program);
    part2(&program);
}

fn part1(program: &Program) {
    let value = run_machine(&program, 12.into(), 2.into());
    println!("{}", value.0);
}

fn part2(program: &Program) {
    let input = run_all_combinations(&program);
    println!("{}", input);
}

fn run_machine(program: &Program, noun: mem::Value, verb: mem::Value) -> mem::Value {
    let mut machine = Machine::default_io(&program);
    machine.memory[mem::NOUN_ADDRESS] = noun;
    machine.memory[mem::VERB_ADDRESS] = verb;
    machine.run();
    machine.memory[mem::Address(0)]
}

fn run_all_combinations(program: &Program) -> isize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let value = run_machine(&program, noun.into(), verb.into());
            if value.0 == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("couldn't find noun & verb");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_known_answers() {
        let input = include_str!("../../input/day02.in");
        let program = input.parse::<Program>().unwrap();
        assert_eq!(run_machine(&program, 12.into(), 2.into()), 5110675.into());
        assert_eq!(run_all_combinations(&program), 4847);
    }
}
