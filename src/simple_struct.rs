use std::num::ParseIntError;

use crate::deserializable::Deserializable;

#[derive(Debug)]
pub struct SimpleType {
  number: u32,
  text: String,
}

impl Deserializable for SimpleType {
    type Err = ParseIntError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let (number, text) = s.split_once(" ").unwrap();
        let number_from_string = number.parse::<u32>()?;

        Ok(SimpleType { number: number_from_string, text: text.to_string()})
    }
}
