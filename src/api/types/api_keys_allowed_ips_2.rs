pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllowedIps2 {
    Clear,
    NoUpdate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AllowedIps2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Clear => serializer.serialize_str("clear"),
            Self::NoUpdate => serializer.serialize_str("no_update"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AllowedIps2 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "clear" => Ok(Self::Clear),
            "no_update" => Ok(Self::NoUpdate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AllowedIps2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear => write!(f, "clear"),
            Self::NoUpdate => write!(f, "no_update"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
