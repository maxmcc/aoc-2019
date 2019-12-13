use num::{BigInt, Integer};
use serde_derive::Deserialize;
use serde_scan::{scan, ScanError};
use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/day12.in");
    let moons = input.parse::<Moons>().unwrap();
    part1(&moons);
    part2(&moons);
}

fn part1(moons: &Moons) {
    let mut moons = moons.clone();
    for _ in 0..1000 {
        moons.step();
    }
    println!("{}", moons.total_energy());
}

fn part2(moons: &Moons) {
    let original = moons.clone();
    let mut moons = moons.clone();
    let mut steps = 0;
    let mut steps_to_repeat = [None; 3];
    let (x, y, z) = loop {
        if let [Some(x), Some(y), Some(z)] = steps_to_repeat {
            break (x, y, z);
        }
        moons.step();
        steps += 1;
        for &axis in &[Axis::X, Axis::Y, Axis::Z] {
            if steps_to_repeat[axis as usize] == None {
                let same_pos = moons.positions(axis) == original.positions(axis);
                let same_vel = moons.velocities(axis) == original.velocities(axis);
                if same_pos && same_vel {
                    steps_to_repeat[axis as usize] = Some(steps);
                }
            }
        }
    };
    let x = BigInt::from(x);
    let y = BigInt::from(y);
    let z = BigInt::from(z);
    println!("{}", x.lcm(&y).lcm(&z))
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
struct Point3D(isize, isize, isize);

impl Point3D {
    fn get(&self, axis: Axis) -> isize {
        match axis {
            Axis::X => self.0,
            Axis::Y => self.1,
            Axis::Z => self.2,
        }
    }

    fn signum(&self) -> Point3D {
        Point3D(self.0.signum(), self.1.signum(), self.2.signum())
    }

    fn magnitude(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl std::ops::Add for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Self::Output {
        Point3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign for Point3D {
    fn add_assign(&mut self, other: Point3D) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Self::Output {
        Point3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign for Point3D {
    fn sub_assign(&mut self, other: Point3D) {
        *self = *self - other;
    }
}

impl FromStr for Point3D {
    type Err = ScanError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        scan!("<x={}, y={}, z={}>" <- string)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Moons {
    positions: [Point3D; 4],
    velocities: [Point3D; 4],
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Moons {
    fn new(positions: &[Point3D]) -> Self {
        let mut p = [Point3D(0, 0, 0); 4];
        p.copy_from_slice(&positions[0..4]);
        Moons {
            positions: p,
            velocities: [Point3D(0, 0, 0); 4],
        }
    }

    fn step(&mut self) {
        self.do_gravity();
        self.do_velocity();
    }

    fn do_gravity(&mut self) {
        for a in 0..4 {
            for b in a..4 {
                let accel = self.positions[b] - self.positions[a];
                self.velocities[a] += accel.signum();
                self.velocities[b] -= accel.signum();
            }
        }
    }

    fn do_velocity(&mut self) {
        for a in 0..4 {
            self.positions[a] += self.velocities[a];
        }
    }

    fn total_energy(&self) -> isize {
        self.positions
            .iter()
            .zip(self.velocities.iter())
            .map(|(pos, vel)| pos.magnitude() * vel.magnitude())
            .sum()
    }

    fn positions(&self, axis: Axis) -> Vec<isize> {
        self.positions.iter().map(|point| point.get(axis)).collect()
    }

    fn velocities(&self, axis: Axis) -> Vec<isize> {
        self.velocities
            .iter()
            .map(|point| point.get(axis))
            .collect()
    }
}

impl FromStr for Moons {
    type Err = !;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let points = string.lines().map(Point3D::from_str).filter_map(Result::ok);
        Ok(Moons::new(&points.collect::<Vec<_>>()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_moons() {
        let input = r#"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
            "#;
        let mut moons = input.parse::<Moons>().unwrap();
        assert_eq!(
            moons,
            Moons {
                positions: [
                    Point3D(-1, 0, 2),
                    Point3D(2, -10, -7),
                    Point3D(4, -8, 8),
                    Point3D(3, 5, -1),
                ],
                velocities: [
                    Point3D(0, 0, 0),
                    Point3D(0, 0, 0),
                    Point3D(0, 0, 0),
                    Point3D(0, 0, 0),
                ]
            }
        );
        moons.step();
        assert_eq!(
            moons,
            Moons {
                positions: [
                    Point3D(2, -1, 1),
                    Point3D(3, -7, -4),
                    Point3D(1, -7, 5),
                    Point3D(2, 2, 0),
                ],
                velocities: [
                    Point3D(3, -1, -1),
                    Point3D(1, 3, 3),
                    Point3D(-3, 1, -3),
                    Point3D(-1, -3, 1),
                ]
            }
        );
        moons.step();
        assert_eq!(
            moons,
            Moons {
                positions: [
                    Point3D(5, -3, -1),
                    Point3D(1, -2, 2),
                    Point3D(1, -4, -1),
                    Point3D(1, -4, 2),
                ],
                velocities: [
                    Point3D(3, -2, -2),
                    Point3D(-2, 5, 6),
                    Point3D(0, 3, -6),
                    Point3D(-1, -6, 2),
                ]
            }
        );

        for _ in 0..8 {
            moons.step();
        }

        assert_eq!(moons.total_energy(), 179);
    }
}
