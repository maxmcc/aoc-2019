#![feature(slice_patterns)]

use std::error::Error;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut memory = input
        .split(',')
        .map(|x| x.parse::<usize>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();
    memory[1] = 12;
    memory[2] = 2;
    run_computer(&mut memory);
    println!("{}", memory[0]);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let memory = input.split(',').map(|x| x.parse::<usize>()).filter_map(std::result::Result::ok).collect::<Vec<_>>();
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_computer(&mut memory);
            if memory[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
    Ok(())
}

fn run_computer(memory: &mut [usize]) {
    let mut cursor = 0;
    loop {
        match memory[cursor..] {
            [1, lhs, rhs, dst, ..] => memory[dst] = memory[lhs] + memory[rhs],
            [2, lhs, rhs, dst, ..] => memory[dst] = memory[lhs] * memory[rhs],
            [99, ..] => break,
            [x, ..] => panic!("unrecognized opcode {}", x),
            [] => panic!("empty slice"),
        }
        cursor += 4;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_computer(&mut memory);
        assert_eq!(memory, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut memory = vec![1, 0, 0, 0, 99];
        run_computer(&mut memory);
        assert_eq!(memory, vec![2, 0, 0, 0, 99]);

        let mut memory = vec![2, 3, 0, 3, 99];
        run_computer(&mut memory);
        assert_eq!(memory, vec![2, 3, 0, 6, 99]);

        let mut memory = vec![2, 4, 4, 5, 99, 0];
        run_computer(&mut memory);
        assert_eq!(memory, vec![2, 4, 4, 5, 99, 9801]);

        let mut memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_computer(&mut memory);
        assert_eq!(memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
