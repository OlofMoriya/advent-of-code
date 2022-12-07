use core::fmt;
use std::error::Error;

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
pub struct Output {
    pub message: Option<String>,
    pub command: Option<String>
}

impl Deserializable for Output {
    type Err = XError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").expect("there will always be on space");
        if left == "$" {
            //command
            return Ok(Output { message: None, command: Some(right.to_string()) }) 
        }
        else {
            //output

            return Ok(Output {
                message: Some(s.to_string()), 
                command: None
            });
        }
        

    }
}

impl Output {
}
