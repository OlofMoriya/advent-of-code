use core::fmt;
use std::error::Error;

use crate::deserializable::Deserializable;

#[derive(Debug)]
pub struct XError {}

impl Error for XError {}
impl fmt::Display for XError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X had an error")
    }
} 

#[derive(Debug)]
pub struct X {
}

impl Deserializable for X {
    type Err = XError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        return Ok(X {});
    }
}

impl X {
}
