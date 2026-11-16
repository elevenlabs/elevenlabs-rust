pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextWeight {
    Normal,
    Bold,
    Num900,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextWeight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Normal => serializer.serialize_str("normal"),
            Self::Bold => serializer.serialize_str("bold"),
            Self::Num900 => serializer.serialize_str("900"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextWeight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "normal" => Ok(Self::Normal),
            "bold" => Ok(Self::Bold),
            "900" => Ok(Self::Num900),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Bold => write!(f, "bold"),
            Self::Num900 => write!(f, "900"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
