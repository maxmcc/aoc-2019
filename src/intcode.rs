//! A virtual machine for the Intcode instruction set.

pub mod mem;
pub mod op;
pub mod vm;

#[cfg(test)]
mod test;

pub use vm::Input as _;
pub use vm::Output as _;
pub use vm::{DefaultInput, DefaultOutput, Machine};

use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Vec<mem::Value>);

impl<I, T> From<I> for Program
where
    I: IntoIterator<Item = T>,
    T: Clone + Into<mem::Value>,
{
    fn from(iter: I) -> Self {
        Program(iter.into_iter().map(Into::into).collect())
    }
}

impl FromStr for Program {
    type Err = !;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let values = string
            .split(',')
            .map(str::parse::<isize>)
            .filter_map(Result::ok)
            .map(mem::Value)
            .collect::<Vec<_>>();
        Ok(Program::from(values))
    }
}
