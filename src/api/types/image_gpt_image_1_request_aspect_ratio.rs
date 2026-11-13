pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GptImage1RequestAspectRatio {
    Num1_1,
    Num3_2,
    Num2_3,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GptImage1RequestAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num1_1 => serializer.serialize_str("1:1"),
            Self::Num3_2 => serializer.serialize_str("3:2"),
            Self::Num2_3 => serializer.serialize_str("2:3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GptImage1RequestAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1:1" => Ok(Self::Num1_1),
            "3:2" => Ok(Self::Num3_2),
            "2:3" => Ok(Self::Num2_3),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GptImage1RequestAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num1_1 => write!(f, "1:1"),
            Self::Num3_2 => write!(f, "3:2"),
            Self::Num2_3 => write!(f, "2:3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
