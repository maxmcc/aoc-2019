use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn fuel_for_mass(mass: isize) -> isize {
    mass / 3 - 2
}

fn fuel_for_module(module_mass: isize) -> isize {
    let mut remaining = module_mass;
    let mut total = 0;
    while remaining > 2 {
        let fuel = std::cmp::max(0, fuel_for_mass(remaining));
        total += fuel;
        remaining = fuel;
    }
    total
}

fn part1(input: &str) -> Result<()> {
    let mut sum = 0;
    for line in input.lines() {
        let mass = line.parse()?;
        sum += fuel_for_mass(mass);
    }
    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut sum = 0;
    for line in input.lines() {
        let mass = line.parse()?;
        sum += fuel_for_module(mass);
    }
    println!("{}", sum);
    Ok(())
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
}
