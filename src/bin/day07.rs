use aoc::intcode::*;

use itertools::Itertools;
use std::iter;

fn main() {
    let input = include_str!("../../input/day07.in");
    let program = input.parse::<Program>().unwrap();
    part1(&program);
    dbg!(part2(&program));
}

fn part1(program: &Program) -> isize {
    let possible_settings = (0..=4).permutations(5);
    let mut running_max = 0;
    for settings in possible_settings {
        let machine = Machine::default_io(&program);
        let mut machines = iter::repeat(machine).take(5).collect::<Vec<_>>();
        for (index, machine) in machines.iter_mut().enumerate() {
            machine.input.queue.push_back(settings[index].into());
        }
        machines[0].input.queue.push_back(0.into());
        for i in 0..5 {
            let prev_machine = &mut machines[(i + 4) % 5];
            let prev_output = prev_machine.output.buffer.drain(..).collect::<Vec<_>>();
            let machine = &mut machines[i];
            machine.input.queue.extend(prev_output);
            machine.run();
        }
        let output = machines[4].output.buffer.first().unwrap();
        running_max = running_max.max(output.0);
    }
    running_max
}

fn part2(program: &Program) -> isize {
    let possible_settings = (5..=9).permutations(5);
    let mut running_max = 0;
    for settings in possible_settings {
        let machine = Machine::default_io(&program);
        let mut machines = iter::repeat(machine).take(5).collect::<Vec<_>>();
        for (index, machine) in machines.iter_mut().enumerate() {
            machine.input.queue.push_back(settings[index].into());
        }
        machines[0].input.queue.push_back(0.into());
        let mut outputs = vec![vec![]; 5];
        'outer: loop {
            for i in 0..5 {
                let machine = &mut machines[i];
                machine.input.queue.extend(outputs[(i + 4) % 5].drain(..));
                let status = machine.run();
                outputs[i].extend(machine.output.buffer.drain(..));
                match status {
                    vm::Status::Halted if i == 4 => {
                        break 'outer;
                    }
                    vm::Status::Halted | vm::Status::Blocked => {
                        continue;
                    }
                    vm::Status::Ready => unreachable!(),
                }
            }
        }
        let output = outputs[4].last().expect("machine 4 should have output");
        running_max = running_max.max(output.0);
    }
    running_max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let program = Program::from(&[
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(part1(&program), 43210);

        let program = Program::from(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert_eq!(part1(&program), 54321);

        let program = Program::from(
            [
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ]
            .iter(),
        );
        assert_eq!(part1(&program), 65210);
    }

    #[test]
    fn test_part2() {
        let program = Program::from(
            [
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ]
            .iter(),
        );
        assert_eq!(part2(&program), 139629729);
    }

    #[test]
    fn test_part2b() {
        let program = Program::from(
            [
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ]
            .iter(),
        );
        assert_eq!(part2(&program), 18216);
    }
}
