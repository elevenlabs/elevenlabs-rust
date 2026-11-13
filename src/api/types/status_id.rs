pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StatusId {
    #[default]
    Num0,
    Num1,
    Num2,
    Num99,
}

impl Serialize for StatusId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num0 => serializer.serialize_i64(0),
            Self::Num1 => serializer.serialize_i64(1),
            Self::Num2 => serializer.serialize_i64(2),
            Self::Num99 => serializer.serialize_i64(99),
        }
    }
}

impl<'de> Deserialize<'de> for StatusId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if value == 0 {
            return Ok(Self::Num0);
        }
        if value == 1 {
            return Ok(Self::Num1);
        }
        if value == 2 {
            return Ok(Self::Num2);
        }
        if value == 99 {
            return Ok(Self::Num99);
        }
        Err(serde::de::Error::custom(format!(
            "unknown StatusId value: {value}"
        )))
    }
}

impl fmt::Display for StatusId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num0 => write!(f, "0"),
            Self::Num1 => write!(f, "1"),
            Self::Num2 => write!(f, "2"),
            Self::Num99 => write!(f, "99"),
        }
    }
}
