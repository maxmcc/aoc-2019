use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut lines = input.lines();

    let first_points = points(lines.next().unwrap());
    let second_points = points(lines.next().unwrap());

    let first: HashSet<_> = first_points.keys().collect();
    let second: HashSet<_> = second_points.keys().collect();

    let intersections = first.intersection(&second);
    let nearest = intersections.map(|p| p.x.abs() + p.y.abs()).min();
    println!("{:?}", nearest);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut lines = input.lines();

    let first_points = points(lines.next().unwrap());
    let second_points = points(lines.next().unwrap());

    let mut shortest: Option<usize> = None;
    for (key, d1) in &first_points {
        if let Some(d2) = second_points.get(key) {
            if let Some(s) = shortest {
                shortest = Some(s.min(d1 + d2));
            } else {
                shortest = Some(d1 + d2);
            }
        }
    }
    println!("{:?}", shortest);
    Ok(())
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
}


fn points(string: &str) -> HashMap<Point, usize> {
    let mut points = HashMap::new();
    let mut travelled = 0;
    let mut current = Point::new(0, 0);

    let segments = string.split(',').map(Segment::from_str);
    for segment in segments {
        match segment {
            Segment::Up(distance) =>
                for _ in 0..distance {
                    travelled += 1;
                    current = Point::new(current.x, current.y + 1);
                    points.insert(current, travelled);
                },
            Segment::Down(distance) =>
                for _ in 0..distance {
                    travelled += 1;
                    current = Point::new(current.x, current.y - 1);
                    points.insert(current, travelled);
                },
            Segment::Left(distance) =>
                for _ in 0..distance {
                    travelled += 1;
                    current = Point::new(current.x - 1, current.y);
                    points.insert(current, travelled);
                },
            Segment::Right(distance) =>
                for _ in 0..distance {
                    travelled += 1;
                    current = Point::new(current.x + 1, current.y);
                    points.insert(current, travelled);
                },
        }
    }
    points.remove(&Point::new(0, 0));
    points
}

#[derive(Clone, Debug)]
enum Segment {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl Segment {
    fn from_str(string: &str) -> Self {
        let distance = string[1..].parse::<isize>().unwrap();
        match string.chars().next().unwrap() {
            'U' => Segment::Up(distance),
            'D' => Segment::Down(distance),
            'L' => Segment::Left(distance),
            'R' => Segment::Right(distance),
            _ => unreachable!(),
        }
    }
}


