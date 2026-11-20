pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HolidayType {
    Closed,
    Override_,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HolidayType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Closed => serializer.serialize_str("closed"),
            Self::Override_ => serializer.serialize_str("override"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HolidayType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "closed" => Ok(Self::Closed),
            "override" => Ok(Self::Override_),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HolidayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed => write!(f, "closed"),
            Self::Override_ => write!(f, "override"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
