pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConvAiWorkspaceStoredSecretConfigType {
    Stored,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConvAiWorkspaceStoredSecretConfigType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stored => serializer.serialize_str("stored"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConvAiWorkspaceStoredSecretConfigType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stored" => Ok(Self::Stored),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConvAiWorkspaceStoredSecretConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stored => write!(f, "stored"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
