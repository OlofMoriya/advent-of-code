use core::fmt;
use std::error::Error;
use std::num::ParseIntError;

use crate::deserializable::Deserializable;

#[derive(Debug)]
pub struct XError {
    message: String
}

impl Error for XError {
}
impl fmt::Display for XError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing had an error, {}", self.message)
    }
} 

#[derive(Debug, Clone)]
pub enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Deserializable for Move {
    type Err = ParseIntError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let (dir, length) = s.split_once(" ").expect("these will always be a direction and lengthn");
        let length = length.parse::<usize>()?; 

        Ok(match dir {
            "U" => Move::Up(length),
            "D" => Move::Down(length),
            "L" => Move::Left(length),
            "R" => Move::Right(length),
            _ => unreachable!()
        })
    }
}

impl Move {
}
