use core::fmt;
use std::error::Error;

use crate::deserializable::Deserializable;
use crate::vec_helper::find_duplicate;

#[derive(Debug)]
pub struct RugsackError {}

impl Error for RugsackError {}
impl fmt::Display for RugsackError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rugsack had an error")
    }
} 

#[derive(Debug, Clone)]
pub struct Rugsack {
    compartment_one : Vec<char>,
    compartment_two : Vec<char>,
    pub items: Vec<char>
}

impl Deserializable for Rugsack {
    type Err = RugsackError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(s.len()/2);
         
        return Ok(Rugsack {
            items: s.chars().collect(),
            compartment_one: left.chars().collect(),
            compartment_two: right.chars().collect()
        });
    }
}

impl Rugsack {
    pub fn find_duplicate(self) -> Vec<char> {
        return find_duplicate(vec!{self.compartment_one, self.compartment_two});
    }
}
