
pub trait Deserializable: Sized {
    type Err;
    fn deserialize(string: &str) -> Result<Self, Self::Err>;
}
