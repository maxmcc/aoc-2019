use itertools::Itertools;
use num::Integer;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/day10.in");
    let asteroids = parse_input(&input);
    part1(&asteroids);
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.trim().chars().enumerate().filter_map(move |(x, item)| {
                if item == '#' {
                    Some(Point(x, y))
                } else if item == '.' {
                    None
                } else {
                    panic!("unexpected character {:#?}", item)
                }
            })
        })
        .collect()
}

fn part1(asteroids: &[Point]) {
    let best = best_point(&asteroids);
    println!("{}", best.1);
}

fn best_point(asteroids: &[Point]) -> (Point, usize) {
    asteroids
        .iter()
        .map(|point| (*point, point.visible_points(asteroids).len()))
        .max_by_key(|(_, visible)| *visible)
        .unwrap()
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point(usize, usize);

impl Point {
    fn slope_to(&self, other: &Point) -> Slope {
        let (&Point(x0, y0), &Point(x1, y1)) = (self, other);
        let dy = y1 as isize - y0 as isize;
        let dx = x1 as isize - x0 as isize;
        Slope::new(dy, dx)
    }
    fn distance_to(&self, other: &Point) -> f64 {
        let (&Point(x0, y0), &Point(x1, y1)) = (self, other);
        let dy = y1 as f64 - y0 as f64;
        let dx = x1 as f64 - x0 as f64;
        dy.hypot(dx)
    }

    fn visible_points<'a, I>(&self, points: I) -> HashSet<Point>
    where
        I: IntoIterator<Item = &'a Point>,
    {
        dbg!(self.group_by_slope(points))
            .values()
            .map(|x| x.first().unwrap())
            .copied()
            .collect()
    }

    fn group_by_slope<'a, I>(&self, points: I) -> HashMap<Slope, Vec<Point>>
    where
        I: IntoIterator<Item = &'a Point>,
    {
        let mut grouped = points
            .into_iter()
            .filter(|point| *point != self)
            .map(|point| (self.slope_to(point), *point))
            .into_group_map();
        for collinear_points in grouped.values_mut() {
            collinear_points.sort_unstable_by(|lhs, rhs| {
                self.distance_to(lhs)
                    .partial_cmp(&self.distance_to(rhs))
                    .unwrap()
            });
        }
        grouped
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Slope(isize, isize);

impl Slope {
    fn new(dy: isize, dx: isize) -> Self {
        let mut slope = Slope(dy, dx);
        slope.reduce();
        assert!(slope.1 >= 0);
        slope
    }

    fn reduce(&mut self) {
        let divisor = self.0.gcd(&self.1);
        self.0 /= divisor;
        self.1 /= divisor;
        if self.1 < 0 {
            self.0 = 0 - self.0;
            self.1 = 0 - self.1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group_by_slope() {
        let points = &[
            Point(0, 0),
            Point(1, 1),
            Point(2, 2),
            Point(5, 0),
            Point(6, 0),
        ];
        let groups = Point(0, 0).group_by_slope(points);
        assert_eq!(groups[&Slope::new(1, 1)], vec![Point(1, 1), Point(2, 2)]);
        assert_eq!(groups[&Slope::new(0, 1)], vec![Point(5, 0), Point(6, 0)]);
    }

    #[test]
    fn test_best_point() {
        let input = r#"
            #####
            #...#
            #.#.#
            #...#
            #####
            "#;
        let asteroids = parse_input(&input);
        assert_eq!(asteroids.len(), 17);
        assert_eq!(best_point(&asteroids), (Point(2, 2), 16));

        let input = r#"
            .#..#
            .....
            #####
            ....#
            ...##
        "#;
        let asteroids = parse_input(&input);
        assert_eq!(asteroids.len(), 10);
        assert_eq!(best_point(&asteroids), (Point(3, 4), 8));

        let input = r#"
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####
        "#;
        let asteroids = parse_input(&input);
        assert_eq!(asteroids.len(), 40);
        assert_eq!(best_point(&asteroids), (Point(5, 8), 33));

        let input = r#"
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.
        "#;
        let asteroids = parse_input(&input);
        assert_eq!(best_point(&asteroids), (Point(1, 2), 35));

        let input = r#"
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..
        "#;
        let asteroids = parse_input(&input);
        assert_eq!(best_point(&asteroids), (Point(6, 3), 41));

        let input = r#"
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
        "#;
        let asteroids = parse_input(&input);
        assert_eq!(best_point(&asteroids), (Point(11, 13), 210));
    }
}
