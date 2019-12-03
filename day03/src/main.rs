use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let first = visited_points(lines.next().unwrap());
    let second = visited_points(lines.next().unwrap());

    let first_points = first.keys().collect::<HashSet<_>>();
    let second_points = second.keys().collect();

    let intersections = first_points.intersection(&second_points);
    let nearest = intersections.map(|p| p.x.abs() + p.y.abs()).min().unwrap();
    println!("{}", nearest);
}

fn part2(input: &str) {
    let mut lines = input.lines();

    let first = visited_points(lines.next().unwrap());
    let second = visited_points(lines.next().unwrap());

    let mut shortest = std::usize::MAX;
    for (point, first_distance) in first {
        match second.get(&point) {
            None => continue,
            Some(second_distance) => {
                let total_distance = first_distance + second_distance;
                shortest = shortest.min(total_distance);
            },
        }
    }
    println!("{}", shortest);
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

fn visited_points(string: &str) -> HashMap<Point, usize> {
    let steps = string.split(',').flat_map(|segment| {
        let direction = segment.chars().next().unwrap();
        let distance = segment[1..].parse::<usize>().unwrap();
        iter::repeat(direction).take(distance)
    });

    let mut points = HashMap::new();
    let mut position = Point { x: 0, y: 0 };
    let mut distance = 0;

    for step in steps {
        distance += 1;
        position = match step {
            'L' => Point { x: position.x - 1, ..position },
            'R' => Point { x: position.x + 1, ..position },
            'U' => Point { y: position.y + 1, ..position },
            'D' => Point { y: position.y - 1, ..position },
            _ => unreachable!(),
        };
        points.insert(position, distance);
    }
    points
}

