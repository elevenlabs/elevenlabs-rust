pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Veo3_1FastRequestResolution {
    Num720P,
    Num1080P,
    Num4K,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Veo3_1FastRequestResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num720P => serializer.serialize_str("720p"),
            Self::Num1080P => serializer.serialize_str("1080p"),
            Self::Num4K => serializer.serialize_str("4K"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Veo3_1FastRequestResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "720p" => Ok(Self::Num720P),
            "1080p" => Ok(Self::Num1080P),
            "4K" => Ok(Self::Num4K),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Veo3_1FastRequestResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num720P => write!(f, "720p"),
            Self::Num1080P => write!(f, "1080p"),
            Self::Num4K => write!(f, "4K"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
