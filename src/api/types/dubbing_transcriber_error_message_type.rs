pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingTranscriberErrorMessageType {
    TranscriberError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingTranscriberErrorMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TranscriberError => serializer.serialize_str("transcriber_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingTranscriberErrorMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transcriber_error" => Ok(Self::TranscriberError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingTranscriberErrorMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TranscriberError => write!(f, "transcriber_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
