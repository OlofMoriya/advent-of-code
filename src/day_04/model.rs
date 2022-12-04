use core::fmt;
use std::error::Error;

use crate::{deserializable::Deserializable, vec_helper::find_duplicate};

#[derive(Debug)]
pub struct XError {}

impl Error for XError {}
impl fmt::Display for XError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X had an error")
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
        
        let (left,right) = s.split_once(",").unwrap();
        let (l_left, l_right) = left.split_once("-").unwrap();
        let (r_left, r_right) = right.split_once("-").unwrap();
        let left_range_left:u32 = l_left.parse::<u32>().unwrap();
        let left_range_right:u32 = l_right.parse::<u32>().unwrap();
        let left_range = (left_range_left..=left_range_right).collect();
        
        let right_range_left:u32 = r_left.parse::<u32>().unwrap();
        let right_range_right:u32 = r_right.parse::<u32>().unwrap();
        let right_range = (right_range_left..=right_range_right).collect();

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
