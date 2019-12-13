use num::Integer;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/day10.in");
    let asteroids = input.parse::<Asteroids>().unwrap();
    part1(&asteroids);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Asteroids(Vec<Point>);

impl FromStr for Asteroids {
    type Err = !;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut points = vec![];
        for (y, row) in string.trim().lines().enumerate() {
            for (x, item) in row.trim().chars().enumerate() {
                if item == '#' {
                    points.push(Point(x, y));
                }
            }
        }
        Ok(Asteroids(points))
    }
}

impl Asteroids {
    fn visible_points(&self, from: &Point) -> HashMap<Slope, Vec<Point>> {
        let mut points_by_slope = HashMap::new();
        for point in &self.0 {
            if point == from {
                continue;
            }
            let slope = from.slope_to(&point);
            let entry = points_by_slope.entry(slope).or_insert(Vec::new());
            entry.push(*point);
        }
        for (_slope, points) in &mut points_by_slope {
            points.sort_unstable_by(|p1, p2| {
                let d1 = from.distance_to(p1);
                let d2 = from.distance_to(p2);
                d1.partial_cmp(&d2).unwrap()
            });
        }
        points_by_slope
    }

    fn most_visible_point(&self) -> (Point, usize) {
        self.0
            .iter()
            .map(|point| (*point, self.visible_points(point).len()))
            .max_by_key(|(_point, visible)| *visible)
            .unwrap()
    }
}

fn part1(asteroids: &Asteroids) {
    let (point, visible) = asteroids.most_visible_point();
    println!("{} from {:?}", visible, point);
}

fn part2(asteroids: &Asteroids) {
    let (point, _) = asteroids.most_visible_point();
    let visible = asteroids.visible_points(&point);
    let mut index = 0;
    loop {
        if index == 200 {}
        index += 1;
    }
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
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Slope(isize, isize);

impl Slope {
    fn new(dy: isize, dx: isize) -> Self {
        let divisor = dy.gcd(&dx);
        Slope(dy / divisor, dx / divisor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_best_point() {
        let input = r#"
            #####
            #...#
            #.#.#
            #...#
            #####
            "#;
        let asteroids = input.parse::<Asteroids>().unwrap();
        assert_eq!(asteroids.most_visible_point(), (Point(2, 2), 16));

        let input = r#"
            .#..#
            .....
            #####
            ....#
            ...##
        "#;
        let asteroids = input.parse::<Asteroids>().unwrap();
        assert_eq!(asteroids.most_visible_point(), (Point(3, 4), 8));

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
        let asteroids = input.parse::<Asteroids>().unwrap();
        assert_eq!(asteroids.most_visible_point(), (Point(5, 8), 33));

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
        let asteroids = input.parse::<Asteroids>().unwrap();
        assert_eq!(asteroids.most_visible_point(), (Point(1, 2), 35));

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
        let asteroids = input.parse::<Asteroids>().unwrap();
        assert_eq!(asteroids.most_visible_point(), (Point(6, 3), 41));

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
        let asteroids = input.parse::<Asteroids>().unwrap();
        assert_eq!(asteroids.most_visible_point(), (Point(11, 13), 210));
    }
}
