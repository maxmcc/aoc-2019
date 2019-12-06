use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

fn main() {
    let input = include_str!("../../input/day03.in");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let first = visited_points(lines.next().unwrap());
    let second = visited_points(lines.next().unwrap());

    let intersections = first.intersection_dedup(&second, |_, _| 0);
    let nearest = intersections.keys().map(Point::manhattan).min().unwrap();
    println!("{}", nearest);
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let first = visited_points(lines.next().unwrap());
    let second = visited_points(lines.next().unwrap());
    let intersections = first.intersection_dedup(&second, |x, y| x + y);
    let shortest = intersections.values().min().unwrap();
    println!("{}", shortest);
}

trait IntersectionDedup {
    type Value;
    fn intersection_dedup<F>(&self, other: &Self, dedup: F) -> Self
    where
        F: Fn(&Self::Value, &Self::Value) -> Self::Value;
}

impl<K: Eq + Hash + Clone, V> IntersectionDedup for HashMap<K, V> {
    type Value = V;
    fn intersection_dedup<F>(&self, other: &Self, dedup: F) -> Self
    where
        F: Fn(&V, &V) -> V,
    {
        let mut intersection = HashMap::new();
        for (key, first_value) in self {
            if let Some(second_value) = other.get(key) {
                let new_value = dedup(first_value, second_value);
                intersection.insert(key.clone(), new_value);
            }
        }
        intersection
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

fn visited_points(string: &str) -> HashMap<Point, usize> {
    let steps = string.split(',').flat_map(|segment| {
        let direction = segment.chars().next().unwrap();
        let distance = segment[1..].parse().unwrap();
        iter::repeat(direction).take(distance)
    });

    let mut points = HashMap::new();
    let mut position = Point::new(0, 0);
    let mut distance = 0;

    for step in steps {
        distance += 1;
        position = match step {
            'L' => Point::new(position.x - 1, position.y),
            'R' => Point::new(position.x + 1, position.y),
            'U' => Point::new(position.x, position.y + 1),
            'D' => Point::new(position.x, position.y - 1),
            _ => unreachable!(),
        };
        points.entry(position).or_insert(distance);
    }
    points.remove(&Point::new(0, 0));
    points
}
