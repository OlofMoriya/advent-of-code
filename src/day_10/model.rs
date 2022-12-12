use std::num::ParseIntError;

use crate::deserializable::Deserializable;

#[derive(Debug, Clone)]
pub enum Instruction {
    AddX(isize),
    Noop(),
}

impl Deserializable for Instruction {
    type Err = ParseIntError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
         Ok(match split.next().unwrap() {
             "addx" => Instruction::AddX(split.next().expect("no bad input").parse::<isize>()?),
             "noop" => Instruction::Noop(),
             _ => unreachable!("No bad input")
        })
    }
}

impl Instruction {
}
