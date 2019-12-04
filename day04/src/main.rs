#![feature(is_sorted)]

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::RangeInclusive;

const INPUT_RANGE: RangeInclusive<usize> = 272091..=815432;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut count = 0;
    for x in INPUT_RANGE {
        if valid_password(x) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part2() {
    let mut count = 0;
    for x in INPUT_RANGE {
        if valid_password2(x) {
            count += 1;
        }
    }
    println!("{}", count);
}

trait Frequencies {
    type Item: Hash + Eq;
    fn frequencies(self) -> HashMap<Self::Item, usize>;
}

impl<I: Iterator> Frequencies for I
where
    I::Item: Hash + Eq,
{
    type Item = I::Item;

    fn frequencies(self) -> HashMap<Self::Item, usize> {
        let mut map = HashMap::new();
        for item in self {
            let counter = map.entry(item).or_insert(0);
            *counter += 1;
        }
        map
    }
}

fn digits(input: usize) -> Vec<usize> {
    let mut result = Vec::<usize>::new();
    let mut input = input;
    while input > 10 {
        result.push(input % 10);
        input /= 10;
    }
    result.push(input);
    result.reverse();
    result
}

fn valid_password(password: usize) -> bool {
    if !INPUT_RANGE.contains(&password) {
        return false;
    }
    let digits = digits(password).to_vec();
    if !digits.iter().is_sorted() {
        return false;
    }
    for window in digits.windows(2) {
        if window[0] == window[1] {
            return true;
        }
    }
    false
}

fn valid_password2(password: usize) -> bool {
    if !INPUT_RANGE.contains(&password) {
        return false;
    }
    let digits = digits(password).to_vec();
    if !digits.iter().is_sorted() {
        return false;
    }
    let counts = digits.iter().frequencies();
    counts.values().any(|x| *x == 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frequencies() {
        let counts = [1, 2, 2, 3, 3, 3].iter().frequencies();
        assert_eq!(counts.get(&0), None);
        assert_eq!(counts[&1], 1);
        assert_eq!(counts[&2], 2);
        assert_eq!(counts[&3], 3);
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits(123456), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_valid_password() {
        assert!(valid_password(333333));
        assert!(!valid_password(445670));
        assert!(!valid_password(345678));
    }
}
