pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InitializeConnectionMultiText {
    Unnamed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InitializeConnectionMultiText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unnamed => serializer.serialize_str(" "),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InitializeConnectionMultiText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            " " => Ok(Self::Unnamed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InitializeConnectionMultiText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unnamed => write!(f, " "),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
