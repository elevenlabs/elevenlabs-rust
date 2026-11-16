pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultichannelOutputStyle {
    Separate,
    Combined,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MultichannelOutputStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Separate => serializer.serialize_str("separate"),
            Self::Combined => serializer.serialize_str("combined"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MultichannelOutputStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "separate" => Ok(Self::Separate),
            "combined" => Ok(Self::Combined),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MultichannelOutputStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Separate => write!(f, "separate"),
            Self::Combined => write!(f, "combined"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
