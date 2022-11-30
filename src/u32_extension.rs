use std::num::ParseIntError;

use crate::deserializable::Deserializable;

impl Deserializable for u32 {
    type Err = ParseIntError;

    fn deserialize(string: &str) -> Result<Self, Self::Err> {
        return Ok(string.parse::<u32>()?);
    }
}
