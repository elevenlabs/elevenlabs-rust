pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Gemini3ProImageRequestResolution {
    Num1K,
    Num2K,
    Num4K,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Gemini3ProImageRequestResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num1K => serializer.serialize_str("1K"),
            Self::Num2K => serializer.serialize_str("2K"),
            Self::Num4K => serializer.serialize_str("4K"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Gemini3ProImageRequestResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1K" => Ok(Self::Num1K),
            "2K" => Ok(Self::Num2K),
            "4K" => Ok(Self::Num4K),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Gemini3ProImageRequestResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num1K => write!(f, "1K"),
            Self::Num2K => write!(f, "2K"),
            Self::Num4K => write!(f, "4K"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
