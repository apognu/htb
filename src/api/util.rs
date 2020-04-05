use serde::{de::Visitor, Deserializer};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct HtbError {
    pub message: String,
}

impl Display for HtbError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HtbError {}

impl HtbError {
    pub fn new<S>(message: S) -> HtbError
    where
        S: Into<String>,
    {
        HtbError {
            message: message.into(),
        }
    }
}

struct StateSuccessVisitor;

impl<'de> Visitor<'de> for StateSuccessVisitor {
    type Value = u8;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("unsigned integer or string")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value as u8)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.parse::<u8>() {
            Ok(value) => self.visit_u8(value),
            Err(_) => Err(E::custom("failed to parse integer")),
        }
    }
}

pub(super) fn int_or_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StateSuccessVisitor)
}
