pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterByCreator {
    Personal,
    Others,
    All,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FilterByCreator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Personal => serializer.serialize_str("personal"),
            Self::Others => serializer.serialize_str("others"),
            Self::All => serializer.serialize_str("all"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FilterByCreator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "personal" => Ok(Self::Personal),
            "others" => Ok(Self::Others),
            "all" => Ok(Self::All),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FilterByCreator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Personal => write!(f, "personal"),
            Self::Others => write!(f, "others"),
            Self::All => write!(f, "all"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
