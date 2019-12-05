#![feature(slice_patterns)]

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut computer = Computer::from_str(input);
    let _ = computer.run(1);
}

fn part2(input: &str) {
    let mut computer = Computer::from_str(input);
    let _ = computer.run(5);
}

#[derive(Clone, Debug)]
struct Computer {
    memory: Vec<isize>,
    instruction_ptr: usize,
}

impl Computer {
    fn from_str(string: &str) -> Self {
        let memory: Vec<_> = string.split(',').map(|x| x.parse().unwrap()).collect();
        Computer::new(&memory)
    }

    fn new(memory: &[isize]) -> Self {
        Computer {
            memory: memory.to_vec(),
            instruction_ptr: 0,
        }
    }

    fn get_parameter_value(&self, param: Parameter) -> isize {
        match param {
            Parameter::PositionMode(addr) => self.memory[addr],
            Parameter::ImmediateMode(value) => value,
        }
    }

    fn run(&mut self, input: isize) -> Vec<isize> {
        let mut output = vec![];
        loop {
            let memory = &self.memory[self.instruction_ptr..];
            let instruction = Instruction::from_memory(memory);
            match instruction {
                Instruction::Add(lhs, rhs, dst) => {
                    let x = self.get_parameter_value(lhs);
                    let y = self.get_parameter_value(rhs);
                    if let Parameter::PositionMode(d) = dst {
                        self.memory[d] = x + y;
                    } else {
                        panic!("found destination in immediate mode");
                    }
                }
                Instruction::Multiply(lhs, rhs, dst) => {
                    let x = self.get_parameter_value(lhs);
                    let y = self.get_parameter_value(rhs);
                    if let Parameter::PositionMode(d) = dst {
                        self.memory[d] = x * y;
                    } else {
                        panic!("found destination in immediate mode");
                    }
                }
                Instruction::Input(dst) => {
                    if let Parameter::PositionMode(d) = dst {
                        self.memory[d] = input;
                    } else {
                        panic!("found destination in immediate mode");
                    }
                }
                Instruction::Output(p) => {
                    let x = self.get_parameter_value(p);
                    output.push(x);
                }
                Instruction::JumpIfTrue(val, dst) => {
                    let x = self.get_parameter_value(val);
                    let d = self.get_parameter_value(dst);
                    if x != 0 {
                        self.instruction_ptr = d as usize;
                    } else {
                        self.instruction_ptr += 3;
                    }
                }
                Instruction::JumpIfFalse(val, dst) => {
                    let x = self.get_parameter_value(val);
                    let d = self.get_parameter_value(dst);
                    if x == 0 {
                        self.instruction_ptr = d as usize;
                    } else {
                        self.instruction_ptr += 3;
                    }
                }
                Instruction::LessThan(lhs, rhs, dst) => {
                    let x = self.get_parameter_value(lhs);
                    let y = self.get_parameter_value(rhs);
                    if let Parameter::PositionMode(d) = dst {
                        self.memory[d] = if x < y { 1 } else { 0 };
                    } else {
                        panic!("found destination in immediate mode");
                    }
                }
                Instruction::Equals(lhs, rhs, dst) => {
                    let x = self.get_parameter_value(lhs);
                    let y = self.get_parameter_value(rhs);
                    if let Parameter::PositionMode(d) = dst {
                        self.memory[d] = if x == y { 1 } else { 0 };
                    } else {
                        panic!("found destination in immediate mode");
                    }
                }
                Instruction::Halt => return output,
            }
            if let Some(offset) = instruction.memory_len() {
                self.instruction_ptr += offset;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Parameter {
    PositionMode(usize),
    ImmediateMode(isize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

impl Instruction {
    fn from_memory(memory: &[isize]) -> Self {
        assert!(memory[0] >= 0);
        let instruction = format!("{:09}", memory[0]).chars().collect::<Vec<_>>();
        let (mode, opcode) = instruction.split_at(instruction.len() - 2);
        let mut parameters = mode
            .iter()
            .rev()
            .chain(std::iter::repeat(&'0'))
            .zip(&memory[1..])
            .map(|(mode, memory)| match mode {
                '0' => Parameter::PositionMode(*memory as usize),
                '1' => Parameter::ImmediateMode(*memory),
                _ => panic!("unrecognized parameter mode"),
            });

        match opcode {
            ['0', '1'] => Instruction::Add(
                parameters.next().unwrap(),
                parameters.next().unwrap(),
                parameters.next().unwrap(),
            ),
            ['0', '2'] => Instruction::Multiply(
                parameters.next().unwrap(),
                parameters.next().unwrap(),
                parameters.next().unwrap(),
            ),
            ['0', '3'] => Instruction::Input(parameters.next().unwrap()),
            ['0', '4'] => Instruction::Output(parameters.next().unwrap()),
            ['0', '5'] => {
                Instruction::JumpIfTrue(parameters.next().unwrap(), parameters.next().unwrap())
            }
            ['0', '6'] => {
                Instruction::JumpIfFalse(parameters.next().unwrap(), parameters.next().unwrap())
            }
            ['0', '7'] => Instruction::LessThan(
                parameters.next().unwrap(),
                parameters.next().unwrap(),
                parameters.next().unwrap(),
            ),
            ['0', '8'] => Instruction::Equals(
                parameters.next().unwrap(),
                parameters.next().unwrap(),
                parameters.next().unwrap(),
            ),
            ['9', '9'] => Instruction::Halt,
            _ => panic!("unrecognized opcode {:?}", opcode),
        }
    }

    fn memory_len(&self) -> Option<usize> {
        match self {
            Instruction::Add { .. } => Some(4),
            Instruction::Multiply { .. } => Some(4),
            Instruction::Input(..) => Some(2),
            Instruction::Output(..) => Some(2),
            Instruction::JumpIfTrue { .. } => None,
            Instruction::JumpIfFalse { .. } => None,
            Instruction::LessThan { .. } => Some(4),
            Instruction::Equals { .. } => Some(4),
            Instruction::Halt => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from_memory() {
        assert_eq!(
            Instruction::from_memory(&[1002, 4, 3, 4]),
            Instruction::Multiply(
                Parameter::PositionMode(4),
                Parameter::ImmediateMode(3),
                Parameter::PositionMode(4),
            )
        );

        assert_eq!(
            Instruction::from_memory(&[101, -1768, 224, 224]),
            Instruction::Add(
                Parameter::ImmediateMode(-1768),
                Parameter::PositionMode(224),
                Parameter::PositionMode(224),
            )
        );
    }

    #[test]
    fn test_8_eq_8_position_mode() {
        let mut computer = Computer::new(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(computer.run(8), [1]);
    }

    #[test]
    fn test_7_eq_8_position_mode_7() {
        let mut computer = Computer::new(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(computer.run(7), [0]);
    }

    #[test]
    fn test_5_lt_8_position_mode() {
        let mut computer = Computer::new(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(computer.run(5), [1]);
    }

    #[test]
    fn test_8_lt_8_position_mode_8() {
        let mut computer = Computer::new(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(computer.run(8), [0]);
    }

    #[test]
    fn test_10_lt_8_position_mode() {
        let mut computer = Computer::new(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(computer.run(10), [0]);
    }

    #[test]
    fn test_8_eq_8_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.run(8), [1]);
    }

    #[test]
    fn test_7_eq_8_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.run(7), [0]);
    }

    #[test]
    fn test_5_lt_8_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.run(5), [1]);
    }

    #[test]
    fn test_8_lt_8_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.run(8), [0]);
    }

    #[test]
    fn test_12_lt_8_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.run(12), [0]);
    }

    #[test]
    fn test_nz_0_position_mode() {
        let mut computer =
            Computer::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        assert_eq!(computer.run(0), [0]);
    }

    #[test]
    fn test_nz_1_position_mode() {
        let mut computer =
            Computer::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        assert_eq!(computer.run(1), [1]);
    }

    #[test]
    fn test_nz_5_position_mode() {
        let mut computer =
            Computer::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        assert_eq!(computer.run(5), [1]);
    }

    #[test]
    fn test_nz_0_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(computer.run(0), [0]);
    }

    #[test]
    fn test_nz_1_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(computer.run(1), [1]);
    }

    #[test]
    fn test_nz_39_immediate_mode() {
        let mut computer = Computer::new(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(computer.run(39), [1]);
    }
}
