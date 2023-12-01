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
pub struct X {
}

impl Deserializable for X {
    type Err = XError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        return Ok(X {
        });
    }
}

impl X {
}
