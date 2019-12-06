#![feature(is_sorted)]

use std::ops::RangeInclusive;

const INPUT_RANGE: RangeInclusive<usize> = 272091..=815432;

fn main() {
    part1();
    part2();
}

fn part1() {
    let count = INPUT_RANGE
        .map(Digits::new)
        .filter(|digits| {
            digits.is_sorted()
                && digits
                    .collect::<Vec<_>>()
                    .groups()
                    .any(|group| group.len() > 1)
        })
        .count();
    println!("{}", count);
}

fn part2() {
    let count = INPUT_RANGE
        .map(Digits::new)
        .filter(|digits| {
            digits.is_sorted()
                && digits
                    .collect::<Vec<_>>()
                    .groups()
                    .any(|group| group.len() == 2)
        })
        .count();
    println!("{}", count);
}

#[derive(Clone, Copy, Debug)]
struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }
        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = self.n / self.divisor;
            self.n %= self.divisor;
            self.divisor /= 10;
            Some(v as u8)
        }
    }
}

struct Groups<'a, T> {
    base: &'a [T],
}

impl<'a, T: PartialEq> Iterator for Groups<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.base.first() {
            let mut index = 1;
            while index < self.base.len() && self.base[index] == *item {
                index += 1;
            }
            let (group, rest) = self.base.split_at(index);
            self.base = rest;
            Some(group)
        } else {
            None
        }
    }
}

trait GroupsImpl<T> {
    fn groups(&self) -> Groups<T>;
}

impl<T> GroupsImpl<T> for [T] {
    fn groups(&self) -> Groups<T> {
        Groups { base: self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_groups() {
        let groups = [1, 1, 1, 1].groups().map(|g| g.to_vec());
        assert!(groups.eq(vec![vec![1, 1, 1, 1]]));

        let groups = [1, 1, 2, 3, 2, 2].groups().map(|g| g.to_vec());
        assert!(groups.eq(vec![vec![1, 1], vec![2], vec![3], vec![2, 2]]));

        let groups = [].groups().map(|g: &[i32]| g.to_vec());
        assert_eq!(groups.count(), 0);
    }
}
