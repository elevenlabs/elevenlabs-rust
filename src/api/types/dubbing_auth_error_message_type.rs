pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingAuthErrorMessageType {
    AuthError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingAuthErrorMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthError => serializer.serialize_str("auth_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingAuthErrorMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auth_error" => Ok(Self::AuthError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingAuthErrorMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthError => write!(f, "auth_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
