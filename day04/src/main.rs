#![feature(is_sorted)]

use std::ops::RangeInclusive;

const INPUT_RANGE: RangeInclusive<usize> = 272091..=815432;

fn main() {
    part1();
    part2();
}

fn part1() {
    let count = INPUT_RANGE
        .filter(|password| check(*password, |group_len| group_len > 1))
        .count();
    println!("{}", count);
}

fn part2() {
    let count = INPUT_RANGE
        .filter(|password| check(*password, |group_len| group_len == 2))
        .count();
    println!("{}", count);
}

fn check(password: usize, group_len_match: impl Fn(usize) -> bool) -> bool {
    let digits = password.to_string().chars().collect::<Vec<_>>();
    digits.is_sorted() && groups(&digits).map(|x| x.len()).any(group_len_match)
}

fn groups<T: PartialEq>(data: &[T]) -> impl Iterator<Item = &[T]> {
    let mut slice_start = 0;
    (1..data.len() + 1).flat_map(move |i| {
        if i == data.len() || data[i - 1] != data[i] {
            let begin = slice_start;
            slice_start = i;
            Some(&data[begin..i])
        } else {
            None
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
}
