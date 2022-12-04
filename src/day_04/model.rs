use core::fmt;
use std::error::Error;

use crate::{deserializable::Deserializable, vec_helper::find_duplicate};

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
pub struct Pair {
    left:Vec<u32>,
    right:Vec<u32>
}

impl Deserializable for Pair {
    type Err = XError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        
        let bounds:Vec<u32> = s.split(",")
            .map(|i| i.split("-").collect::<Vec<&str>>())
            .flatten()
            .map(|s:&str| s.parse::<u32>().expect("Could not parse value to u32"))
            .collect();

        if bounds.len() != 4 {
            return Err(XError{message: "Incorrect split happened".to_string()});
        }
        let left_range = (bounds[0]..=bounds[1]).collect();
        let right_range = (bounds[2]..=bounds[3]).collect();


        return Ok(Pair {
           left: left_range,
           right: right_range
        });
    }
}

impl Pair {
    pub fn is_completly_overlapped(&self) -> bool {
       if self.left.first() >= self.right.first() && self.left.last() <= self.right.last() {
            return true;
       } else if self.right.first() >= self.left.first() && self.right.last() <= self.left.last(){
            return true;
       }
       return false;
    }

    pub fn has_overlap(self) -> bool {
        return find_duplicate(vec!{self.left, self.right}).len() > 0;
    }
}
