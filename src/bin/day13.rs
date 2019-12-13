use aoc::intcode::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day13.in");
    let program = input.parse::<Program>().unwrap();
    part1(&program);
    part2(&program);
}

fn part1(program: &Program) {
    let mut machine = Machine::<DefaultInput, Game>::new(&program);
    machine.run();
    let blocks = machine
        .output
        .tiles
        .values()
        .filter(|tile| **tile == Tile::Block)
        .count();
    println!("{}", blocks);
}

fn part2(program: &Program) {
    let mut machine = Machine::<DefaultInput, Game>::new(&program);
    machine.memory[mem::Address(0)] = 2.into();
    loop {
        match machine.run() {
            vm::Status::Halted => break,
            vm::Status::Blocked => machine
                .input
                .queue
                .push_back(machine.output.read_input().unwrap()),
            vm::Status::Ready => continue,
        }
    }
    println!("{}", machine.output.score);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(usize, usize);

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, Hash, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    pending_output: Vec<mem::Value>,
    score: isize,
    tiles: HashMap<Point, Tile>,
    paddle_loc: Point,
    ball_loc: Point,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            pending_output: Vec::new(),
            tiles: HashMap::new(),
            score: 0,
            paddle_loc: Point(0, 0),
            ball_loc: Point(0, 0),
        }
    }
}

impl vm::Output for Game {
    fn write_output(&mut self, value: mem::Value) {
        self.pending_output.push(value);
        match self.pending_output.as_slice() {
            [mem::Value(-1), mem::Value(0), score] => {
                self.score = score.0;
                self.pending_output.clear();
            }
            [x, y, tile_id] => {
                let point = Point(x.0 as usize, y.0 as usize);
                let tile = Tile::from_isize(tile_id.0).unwrap();
                self.tiles.insert(point, tile);
                match tile {
                    Tile::Ball => self.ball_loc = point,
                    Tile::Paddle => self.paddle_loc = point,
                    _ => (),
                }
                self.pending_output.clear();
            }
            _ => (),
        }
    }

    fn output_ready(&self) -> bool {
        !self.pending_output.is_empty()
    }
}

impl vm::Input for Game {
    fn read_input(&mut self) -> Option<mem::Value> {
        let paddle = self.paddle_loc.0 as isize;
        let ball = self.ball_loc.0 as isize;
        Some((ball - paddle).signum().into())
    }
}
