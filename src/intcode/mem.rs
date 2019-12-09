use super::{op, Program};

use std::iter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Value(pub isize);

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, other: Self) -> Self {
        Value(self.0 + other.0)
    }
}

impl std::ops::Mul for Value {
    type Output = Value;
    fn mul(self, other: Self) -> Self {
        Value(self.0 * other.0)
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        self.0 != 0
    }
}

impl From<isize> for Value {
    fn from(i: isize) -> Self {
        Self(i)
    }
}

impl From<&isize> for Value {
    fn from(i: &isize) -> Self {
        Self(*i)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        match b {
            true => Self(1),
            false => Self(0),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Address(pub usize);

pub const NOUN_ADDRESS: Address = Address(1);
pub const VERB_ADDRESS: Address = Address(2);

impl std::ops::Add<isize> for Address {
    type Output = Address;
    fn add(self, other: isize) -> Address {
        Address((self.0 as isize + other) as usize)
    }
}

impl std::ops::AddAssign<isize> for Address {
    fn add_assign(&mut self, other: isize) {
        *self = *self + other;
    }
}

impl From<usize> for Address {
    fn from(u: usize) -> Self {
        Address(u)
    }
}

impl From<Value> for Address {
    fn from(value: Value) -> Self {
        Address(value.0 as usize)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    pub values: Vec<Value>,
    pub rel_base: isize,
}

impl Memory {
    pub fn read_instruction(&self, start: Address) -> op::Instruction {
        let mem_slice = &self.values[start.0..];
        mem_slice.into()
    }

    pub fn store(&mut self, value: Value, store: op::Store) {
        let base = self.rel_base;
        match store {
            op::Store::Position(address) => self[address] = value,
            op::Store::Relative(address) => self[address + base] = value,
        }
    }

    pub fn load(&self, load: op::Load) -> Value {
        match load {
            op::Load::Position(address) => self[address],
            op::Load::Immediate(value) => value,
            op::Load::Relative(address) => self[address + self.rel_base],
        }
    }
}

impl From<&Program> for Memory {
    fn from(program: &Program) -> Self {
        let values = program
            .0
            .iter()
            .copied()
            .chain(iter::repeat(0.into()).take(16384))
            .collect();
        Memory {
            values: values,
            rel_base: 0,
        }
    }
}

impl<S, T> From<S> for Memory
where
    S: IntoIterator<Item = T>,
    T: Into<Value>,
{
    fn from(slice: S) -> Self {
        let values = slice
            .into_iter()
            .map(Into::into)
            .chain(iter::repeat(0.into()).take(16394))
            .collect();
        Memory {
            values: values,
            rel_base: 0,
        }
    }
}

impl std::ops::Index<Address> for Memory {
    type Output = Value;
    fn index(&self, addr: Address) -> &Self::Output {
        &self.values[addr.0]
    }
}

impl std::ops::IndexMut<Address> for Memory {
    fn index_mut(&mut self, addr: Address) -> &mut Self::Output {
        &mut self.values[addr.0]
    }
}
