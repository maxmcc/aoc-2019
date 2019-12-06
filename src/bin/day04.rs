#![feature(is_sorted)]

extern crate digits_iterator;
use digits_iterator::*;

extern crate slice_group_by;
use slice_group_by::GroupBy;

use std::ops::RangeInclusive;

const INPUT_RANGE: RangeInclusive<usize> = 272091..=815432;

fn main() {
    part1();
    part2();
}

fn part1() {
    let count = INPUT_RANGE.filter(valid_password_1).count();
    println!("{}", count);
}

fn part2() {
    let count = INPUT_RANGE.filter(valid_password_2).count();
    println!("{}", count);
}

fn valid_password_1(password: &usize) -> bool {
    let digits = password.digits().collect::<Vec<_>>();
    digits.is_sorted() && digits.linear_group().any(|group| group.len() > 1)
}

fn valid_password_2(password: &usize) -> bool {
    let digits = password.digits().collect::<Vec<_>>();
    digits.is_sorted() && digits.linear_group().any(|group| group.len() == 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_password_1() {
        assert!(valid_password_1(&112235));
        assert!(valid_password_1(&123445));
        assert!(!valid_password_1(&233424));
        assert!(!valid_password_1(&123456));
    }

    #[test]
    fn test_valid_password_2() {
        assert!(valid_password_2(&112235));
        assert!(!valid_password_2(&122235));
    }
}
