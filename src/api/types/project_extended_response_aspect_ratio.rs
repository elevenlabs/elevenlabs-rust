pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectExtendedResponseAspectRatio {
    Num16_9,
    Num9_16,
    Num4_5,
    Num1_1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectExtendedResponseAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num16_9 => serializer.serialize_str("16:9"),
            Self::Num9_16 => serializer.serialize_str("9:16"),
            Self::Num4_5 => serializer.serialize_str("4:5"),
            Self::Num1_1 => serializer.serialize_str("1:1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectExtendedResponseAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "16:9" => Ok(Self::Num16_9),
            "9:16" => Ok(Self::Num9_16),
            "4:5" => Ok(Self::Num4_5),
            "1:1" => Ok(Self::Num1_1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectExtendedResponseAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num16_9 => write!(f, "16:9"),
            Self::Num9_16 => write!(f, "9:16"),
            Self::Num4_5 => write!(f, "4:5"),
            Self::Num1_1 => write!(f, "1:1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
