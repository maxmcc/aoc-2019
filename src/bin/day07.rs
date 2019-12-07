#![allow(unused_imports, dead_code)]

use aoc::intcode::*;

use std::collections::{HashSet, VecDeque};
use std::iter;

fn main() {
    let input = include_str!("../../input/day07.in");
    println!("{}", part1(&Computer::from_str(&input)));
    //    println!("{}", part2(&Computer::from_str(&input)));
}

fn part1(computer: &Computer) -> isize {
    let mut running_max = 0;
    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    for e in 0..=4 {
                        let mut comps = iter::repeat(computer.clone()).take(5).collect::<Vec<_>>();
                        let setting = vec![a, b, c, d, e];
                        let s = setting.iter().collect::<HashSet<_>>();
                        if s.len() != 5 {
                            continue;
                        }

                        let mut prev_out = 0;
                        for i in 0..5 {
                            let out = comps[i].run(vec![setting[i], prev_out]);
                            prev_out = out[0];
                        }
                        running_max = running_max.max(prev_out);
                    }
                }
            }
        }
    }
    running_max
}

fn part2(computer: &Computer) -> isize {
    let mut running_max = 0;
    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    for e in 5..=9 {
                        let mut comps = iter::repeat(computer.clone()).take(5).collect::<Vec<_>>();
                        let setting = vec![a, b, c, d, e];
                        let s = setting.iter().collect::<HashSet<_>>();
                        if s.len() != 5 {
                            continue;
                        }
                        unimplemented!()
                    }
                }
            }
        }
    }
    running_max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let computer = Computer::new(&[
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(part1(&computer), 43210);

        let computer = Computer::new(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert_eq!(part1(&computer), 54321);

        let computer = Computer::new(&[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        assert_eq!(part1(&computer), 65210);
    }

    #[test]
    fn test_part2() {
        let computer = Computer::new(&[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        assert_eq!(part2(&computer), 139629729);
    }

    #[test]
    fn test_part2b() {
        let computer = Computer::new(&[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        assert_eq!(part2(&computer), 18216);
    }
}
