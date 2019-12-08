fn main() {
    let input = include_str!("../../input/day01.in");
    let masses = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    part1(&masses);
    part2(&masses);
}

fn part1<'a>(masses: impl IntoIterator<Item = &'a isize>) {
    let sum = sum_masses(masses, fuel_for_mass);
    println!("{}", sum);
}

fn part2<'a>(masses: impl IntoIterator<Item = &'a isize>) {
    let sum = sum_masses(masses, fuel_for_module);
    println!("{}", sum);
}

fn sum_masses<'a>(
    masses: impl IntoIterator<Item = &'a isize>,
    f: impl Fn(isize) -> isize,
) -> isize {
    masses.into_iter().copied().map(f).sum()
}

fn fuel_for_mass(mass: isize) -> isize {
    mass / 3 - 2
}

fn fuel_for_module(module_mass: isize) -> isize {
    let mut remaining = module_mass;
    let mut total = 0;
    while remaining > 2 {
        let fuel = fuel_for_mass(remaining).max(0);
        total += fuel;
        remaining = fuel;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_fuel_for_module() {
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 966);
        assert_eq!(fuel_for_module(100756), 50346);
    }

    #[test]
    fn test_known_answers() {
        let input = include_str!("../../input/day01.in");
        let masses = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sum_masses(&masses, fuel_for_mass), 3262991);
        assert_eq!(sum_masses(&masses, fuel_for_module), 4891620);
    }
}
