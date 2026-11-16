pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextBlendMode {
    Normal,
    Difference,
    Multiply,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextBlendMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Normal => serializer.serialize_str("normal"),
            Self::Difference => serializer.serialize_str("difference"),
            Self::Multiply => serializer.serialize_str("multiply"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextBlendMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "normal" => Ok(Self::Normal),
            "difference" => Ok(Self::Difference),
            "multiply" => Ok(Self::Multiply),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Difference => write!(f, "difference"),
            Self::Multiply => write!(f, "multiply"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
