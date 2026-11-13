pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BytedanceSeedream5LiteRequestAspectRatio {
    Auto,
    Num1_1,
    Num3_4,
    Num16_9,
    Num4_3,
    Num9_16,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BytedanceSeedream5LiteRequestAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Num1_1 => serializer.serialize_str("1:1"),
            Self::Num3_4 => serializer.serialize_str("3:4"),
            Self::Num16_9 => serializer.serialize_str("16:9"),
            Self::Num4_3 => serializer.serialize_str("4:3"),
            Self::Num9_16 => serializer.serialize_str("9:16"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BytedanceSeedream5LiteRequestAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "1:1" => Ok(Self::Num1_1),
            "3:4" => Ok(Self::Num3_4),
            "16:9" => Ok(Self::Num16_9),
            "4:3" => Ok(Self::Num4_3),
            "9:16" => Ok(Self::Num9_16),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BytedanceSeedream5LiteRequestAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Num1_1 => write!(f, "1:1"),
            Self::Num3_4 => write!(f, "3:4"),
            Self::Num16_9 => write!(f, "16:9"),
            Self::Num4_3 => write!(f, "4:3"),
            Self::Num9_16 => write!(f, "9:16"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
