use super::mem;

use num::{FromPrimitive, Integer};
use num_derive::FromPrimitive;
use std::ops::{Add, Mul, Not};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Arith(Opcode, Load, Load, Store),
    CondJump(Opcode, Load, Load),
    Input(Store),
    Output(Load),
    SetRelBase(Load),
    Halt,
}

impl Instruction {
    pub fn opcode(&self) -> Opcode {
        match self {
            Instruction::Arith(opcode, ..) => *opcode,
            Instruction::CondJump(opcode, ..) => *opcode,
            Instruction::Input(..) => Opcode::Input,
            Instruction::Output(..) => Opcode::Output,
            Instruction::SetRelBase(..) => Opcode::SetRelBase,
            Instruction::Halt => Opcode::Halt,
        }
    }
}

impl<'a, I, T> From<I> for Instruction
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Clone + Into<mem::Value>,
{
    fn from(iter: I) -> Self {
        let mut values = iter.into_iter().cloned().map(Into::into);
        let first = values.next().unwrap();
        let mut parameters = first.parameter_modes().zip(values);

        let opcode = first.opcode();
        match opcode {
            Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => {
                Instruction::Arith(
                    opcode,
                    parameters.next().unwrap().into(),
                    parameters.next().unwrap().into(),
                    parameters.next().unwrap().into(),
                )
            }
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => Instruction::CondJump(
                opcode,
                parameters.next().unwrap().into(),
                parameters.next().unwrap().into(),
            ),
            Opcode::Input => Instruction::Input(parameters.next().unwrap().into()),
            Opcode::Output => Instruction::Output(parameters.next().unwrap().into()),
            Opcode::SetRelBase => Instruction::SetRelBase(parameters.next().unwrap().into()),
            Opcode::Halt => Instruction::Halt,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Load {
    Position(mem::Address),
    Immediate(mem::Value),
    Relative(mem::Address),
}

impl From<(ParameterMode, mem::Value)> for Load {
    fn from((mode, value): (ParameterMode, mem::Value)) -> Self {
        match mode {
            ParameterMode::Position => Load::Position(value.into()),
            ParameterMode::Immediate => Load::Immediate(value),
            ParameterMode::Relative => Load::Relative(value.into()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Store {
    Position(mem::Address),
    Relative(mem::Address),
}

impl From<(ParameterMode, mem::Value)> for Store {
    fn from((mode, value): (ParameterMode, mem::Value)) -> Self {
        match mode {
            ParameterMode::Position => Store::Position(value.into()),
            ParameterMode::Immediate => panic!("cannot create store in immediate mode"),
            ParameterMode::Relative => Store::Relative(value.into()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    SetRelBase = 9,
    Halt = 99,
}

impl Opcode {
    pub fn len(&self) -> usize {
        match self {
            Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => 4,
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => 3,
            Opcode::Input | Opcode::Output => 2,
            Opcode::SetRelBase => 2,
            Opcode::Halt => 1,
        }
    }

    pub fn arith_fn(&self) -> fn(mem::Value, mem::Value) -> mem::Value {
        match self {
            Opcode::Add => mem::Value::add,
            Opcode::Multiply => mem::Value::mul,
            Opcode::LessThan => |x, y| x.lt(&y).into(),
            Opcode::Equals => |x, y| x.eq(&y).into(),
            _ => panic!("opcode {:?} is not arithmetic", self),
        }
    }

    pub fn cond_jump_fn(&self) -> fn(mem::Value) -> bool {
        match self {
            Opcode::JumpIfTrue => |x| x.into(),
            Opcode::JumpIfFalse => |x| bool::not(x.into()),
            _ => panic!("opcode {:?} is not a jump", self),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl mem::Value {
    fn opcode(&self) -> Opcode {
        let last_two_digits = self.0 as usize % 100;
        Opcode::from_usize(last_two_digits).unwrap()
    }

    fn parameter_modes(&self) -> impl Iterator<Item = ParameterMode> {
        let mut state = self.0 as usize / 100;
        std::iter::from_fn(move || {
            if state == 0 {
                Some(ParameterMode::Position)
            } else {
                let (div, rem) = state.div_rem(&10);
                state = div;
                Some(ParameterMode::from_usize(rem).unwrap())
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opcode_from_value() {
        assert_eq!(mem::Value(1).opcode(), Opcode::Add);
        assert_eq!(mem::Value(101).opcode(), Opcode::Add);
        assert_eq!(mem::Value(1101).opcode(), Opcode::Add);
        assert_eq!(mem::Value(10101).opcode(), Opcode::Add);

        assert_eq!(mem::Value(108).opcode(), Opcode::Equals);
        assert_eq!(mem::Value(99).opcode(), Opcode::Halt);
        assert_eq!(mem::Value(199).opcode(), Opcode::Halt);
    }

    #[test]
    fn test_parameter_modes_from_value() {
        let position = ParameterMode::Position;
        let immediate = ParameterMode::Immediate;
        let relative = ParameterMode::Relative;
        fn modes(value: isize, len: usize) -> Vec<ParameterMode> {
            mem::Value(value).parameter_modes().take(len).collect()
        }
        assert_eq!(modes(1, 3), [position, position, position]);
        assert_eq!(modes(99, 3), [position, position, position]);
        assert_eq!(modes(101, 3), [immediate, position, position]);
        assert_eq!(modes(1101, 3), [immediate, immediate, position]);
        assert_eq!(modes(11101, 3), [immediate, immediate, immediate]);
        assert_eq!(modes(10101, 3), [immediate, position, immediate]);

        assert_eq!(modes(20101, 3), [immediate, position, relative]);
        assert_eq!(modes(203, 1), [relative]);
    }

    #[test]
    fn test_arith_instruction_from_memory() {
        assert_eq!(
            Instruction::from(&[01101, 2, 6, 3]),
            Instruction::Arith(
                Opcode::Add,
                Load::Immediate(2.into()),
                Load::Immediate(6.into()),
                Store::Position(3.into())
            )
        );
        assert_eq!(
            Instruction::from(&[1002, 4, 3, 4, 0, 0]),
            Instruction::Arith(
                Opcode::Multiply,
                Load::Position(4.into()),
                Load::Immediate(3.into()),
                Store::Position(4.into())
            )
        );
        assert_eq!(
            Instruction::from(&[0107, -4, 3, 12]),
            Instruction::Arith(
                Opcode::LessThan,
                Load::Immediate((-4).into()),
                Load::Position(3.into()),
                Store::Position(12.into())
            )
        );
        assert_eq!(
            Instruction::from(&[8, 2, 2, 9]),
            Instruction::Arith(
                Opcode::Equals,
                Load::Position(2.into()),
                Load::Position(2.into()),
                Store::Position(9.into())
            )
        );
    }

    #[test]
    fn test_cond_jump_instruction_from_memory() {
        assert_eq!(
            Instruction::from(&[105, 1, 0]),
            Instruction::CondJump(
                Opcode::JumpIfTrue,
                Load::Immediate(1.into()),
                Load::Position(0.into())
            )
        );
        assert_eq!(
            Instruction::from(&[6, 4, 3]),
            Instruction::CondJump(
                Opcode::JumpIfFalse,
                Load::Position(4.into()),
                Load::Position(3.into())
            )
        );
    }

    #[test]
    fn test_io_instruction_from_memory() {
        assert_eq!(
            Instruction::from(&[3, 1, 3, 3, 4]),
            Instruction::Input(Store::Position(1.into()))
        );
        assert_eq!(
            Instruction::from(&[4, 1]),
            Instruction::Output(Load::Position(1.into()))
        );
        assert_eq!(
            Instruction::from(&[203, 0]),
            Instruction::Input(Store::Relative(0.into()))
        );
    }

    #[test]
    fn test_set_rel_base_instruction_from_memory() {
        assert_eq!(
            Instruction::from(&[109, 6]),
            Instruction::SetRelBase(Load::Immediate(6.into()))
        );
    }

    #[test]
    fn test_halt_instruction_from_memory() {
        assert_eq!(Instruction::from(&[99, -1, 20]), Instruction::Halt);
    }
}
