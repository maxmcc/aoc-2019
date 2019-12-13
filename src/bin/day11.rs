use aoc::intcode::*;

use std::collections::HashSet;

const BLACK: mem::Value = mem::Value(0);
const WHITE: mem::Value = mem::Value(1);

const LEFT_TURN: mem::Value = mem::Value(0);
const RIGHT_TURN: mem::Value = mem::Value(1);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&mut self, value: mem::Value) {
        *self = match value {
            LEFT_TURN => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            RIGHT_TURN => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Robot {
    machine: Machine,
    position: Point,
    heading: Direction,
    white_panels: HashSet<Point>,
    painted_panels: HashSet<Point>,
}

impl Robot {
    fn new(program: &Program, start: mem::Value) -> Self {
        let mut robot = Robot {
            machine: Machine::default_io(&program),
            position: Point(0, 0),
            heading: Direction::Up,
            white_panels: HashSet::new(),
            painted_panels: HashSet::new(),
        };
        if start == WHITE {
            robot.white_panels.insert(Point(0, 0));
        }
        robot
    }

    fn paint(&mut self, value: mem::Value) {
        self.painted_panels.insert(self.position);
        match value {
            BLACK => {
                self.white_panels.remove(&self.position);
            }
            WHITE => {
                self.white_panels.insert(self.position);
            }
            _ => unreachable!(),
        }
    }

    fn turn_and_advance(&mut self, value: mem::Value) {
        self.heading.turn(value);
        match self.heading {
            Direction::Up => self.position.1 += 1,
            Direction::Down => self.position.1 -= 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }

    fn run(&mut self) {
        loop {
            let current_color = if self.white_panels.contains(&self.position) {
                WHITE
            } else {
                BLACK
            };
            match self.machine.step() {
                vm::Status::Ready if self.machine.output.buffer.len() == 2 => {
                    let color = self.machine.output.buffer[0];
                    let turn = self.machine.output.buffer[1];
                    self.machine.output.buffer.clear();
                    self.paint(color);
                    self.turn_and_advance(turn);
                }
                vm::Status::Ready => {
                    continue;
                }
                vm::Status::Blocked => {
                    self.machine.input.queue.push_back(current_color);
                    continue;
                }
                vm::Status::Halted => {
                    break;
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../../input/day11.in");
    let program = input.parse::<Program>().unwrap();
    let mut robot = Robot::new(&program, WHITE);
    robot.run();
    println!("{:?}", robot.white_panels);
}

#[cfg(test)]
mod test {
    use super::*;
}
