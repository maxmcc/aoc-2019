use super::mem::Memory;
use super::op::Instruction;
use crate::intcode::*;

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Ready,
    Blocked,
    Halted,
}

pub trait Input {
    fn read_input(&mut self) -> Option<mem::Value>;
}

pub trait Output {
    fn write_output(&mut self, value: mem::Value);
    fn output_ready(&self) -> bool;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DefaultInput {
    pub queue: VecDeque<mem::Value>,
}

impl Input for DefaultInput {
    fn read_input(&mut self) -> Option<mem::Value> {
        self.queue.pop_front()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DefaultOutput {
    pub buffer: Vec<mem::Value>,
}

impl Output for DefaultOutput {
    fn write_output(&mut self, value: mem::Value) {
        self.buffer.push(value);
    }

    fn output_ready(&self) -> bool {
        !self.buffer.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Machine<I = DefaultInput, O = DefaultOutput>
where
    I: Input,
    O: Output,
{
    pub status: Status,
    pub memory: Memory,
    pub ins_ptr: mem::Address,
    pub input: I,
    pub output: O,
}

impl Machine<DefaultInput, DefaultOutput> {
    pub fn default_io(program: &Program) -> Self {
        Machine::new(program)
    }
}

impl<I, O> Machine<I, O>
where
    I: Input + Default,
    O: Output + Default,
{
    pub fn new(program: &Program) -> Self {
        Machine {
            status: Status::Ready,
            memory: program.0.clone().into(),
            ins_ptr: mem::Address(0),
            input: I::default(),
            output: O::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InsPtrUpdate {
    Jump(mem::Address),
    Advance(mem::Offset),
}

impl<I: Input, O: Output> Machine<I, O> {
    pub fn step(&mut self) -> Status {
        if let Status::Halted = self.status {
            return self.status;
        }
        let instruction = self.memory.read_instruction(self.ins_ptr);
        let update = match instruction {
            Instruction::Arith(opcode, load_lhs, load_rhs, store_result) => {
                let lhs = self.memory.load(load_lhs);
                let rhs = self.memory.load(load_rhs);
                let result = opcode.arith_fn()(lhs, rhs);
                self.memory.store(result, store_result);
                InsPtrUpdate::Advance(opcode.len())
            }
            Instruction::CondJump(opcode, load_x, load_addr) => {
                let x = self.memory.load(load_x);
                let addr = self.memory.load(load_addr).into();
                if opcode.cond_jump_fn()(x) {
                    InsPtrUpdate::Jump(addr)
                } else {
                    InsPtrUpdate::Advance(opcode.len())
                }
            }
            Instruction::Input(store_input) => {
                if let Some(input) = self.input.read_input() {
                    self.memory.store(input, store_input);
                    InsPtrUpdate::Advance(instruction.opcode().len())
                } else {
                    self.status = Status::Blocked;
                    return self.status;
                }
            }
            Instruction::Output(load_output) => {
                let output = self.memory.load(load_output);
                self.output.write_output(output);
                InsPtrUpdate::Advance(instruction.opcode().len())
            }
            Instruction::SetRelBase(load_addr) => {
                let addr = self.memory.load(load_addr);
                self.memory.rel_base += mem::Offset::from(addr);
                InsPtrUpdate::Advance(instruction.opcode().len())
            }
            Instruction::Halt => {
                self.status = Status::Halted;
                return self.status;
            }
        };
        match update {
            InsPtrUpdate::Jump(address) => self.ins_ptr = address,
            InsPtrUpdate::Advance(amount) => self.ins_ptr += amount,
        }
        Status::Ready
    }

    pub fn run(&mut self) -> Status {
        loop {
            match self.step() {
                Status::Ready => continue,
                stopped => return stopped,
            }
        }
    }
}
