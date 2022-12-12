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
    pub fn move_point(&self, head: &(isize, isize), tail: &mut (isize, isize)) -> (String, Option<Move>) {
        let mut side_effect: Option<Move> = None;
        match self {
            Move::Up(_) => {
                if tail.1.abs_diff(head.1) == 2 {
                    tail.1 -= 1;
                    if tail.0 != head.0 {
                        //side_effect = Some(if tail.0 < head.0 {Move::Right(1)} else {Move::Left(1)});
                        tail.0 = head.0;
                    } 
                }
            }
            Move::Down(_) => {
                if (tail.1).abs_diff(head.1) == 2 {
                    tail.1 += 1;
                    if tail.0 != head.0 {
                        //side_effect = Some(if tail.0 < head.0 {Move::Right(1)} else {Move::Left(1)});
                        tail.0 = head.0;
                    } 
                }
            },
            Move::Left(_) => {
                if (tail.0).abs_diff(head.0) == 2 {
                    tail.0 -= 1;
                    if tail.1 != head.1 {
                        //side_effect = Some(if tail.1 < head.1 {Move::Down(1)} else {Move::Up(1)});
                        tail.1 = head.1;
                    } 
                }
            },
            Move::Right(_) => {
                if (tail.0).abs_diff(head.0) == 2 {
                    tail.0 += 1;
                    if tail.1 != head.1 {
                        //side_effect = Some(if tail.1 < head.1 {Move::Down(1)} else {Move::Up(1)});
                        tail.1 = head.1;
                    } 
                }
            },
        };
        //println!("move {:?}, passes possition {},{}", self, tail.1, tail.0);
        return (format!("{},{}", tail.0, tail.1), side_effect);
    }
}
