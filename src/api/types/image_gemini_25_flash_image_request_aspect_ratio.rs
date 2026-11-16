pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Gemini25FlashImageRequestAspectRatio {
    Auto,
    Num1_1,
    Num2_3,
    Num3_2,
    Num3_4,
    Num4_3,
    Num4_5,
    Num5_4,
    Num9_16,
    Num16_9,
    Num21_9,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Gemini25FlashImageRequestAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Num1_1 => serializer.serialize_str("1:1"),
            Self::Num2_3 => serializer.serialize_str("2:3"),
            Self::Num3_2 => serializer.serialize_str("3:2"),
            Self::Num3_4 => serializer.serialize_str("3:4"),
            Self::Num4_3 => serializer.serialize_str("4:3"),
            Self::Num4_5 => serializer.serialize_str("4:5"),
            Self::Num5_4 => serializer.serialize_str("5:4"),
            Self::Num9_16 => serializer.serialize_str("9:16"),
            Self::Num16_9 => serializer.serialize_str("16:9"),
            Self::Num21_9 => serializer.serialize_str("21:9"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Gemini25FlashImageRequestAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "1:1" => Ok(Self::Num1_1),
            "2:3" => Ok(Self::Num2_3),
            "3:2" => Ok(Self::Num3_2),
            "3:4" => Ok(Self::Num3_4),
            "4:3" => Ok(Self::Num4_3),
            "4:5" => Ok(Self::Num4_5),
            "5:4" => Ok(Self::Num5_4),
            "9:16" => Ok(Self::Num9_16),
            "16:9" => Ok(Self::Num16_9),
            "21:9" => Ok(Self::Num21_9),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Gemini25FlashImageRequestAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Num1_1 => write!(f, "1:1"),
            Self::Num2_3 => write!(f, "2:3"),
            Self::Num3_2 => write!(f, "3:2"),
            Self::Num3_4 => write!(f, "3:4"),
            Self::Num4_3 => write!(f, "4:3"),
            Self::Num4_5 => write!(f, "4:5"),
            Self::Num5_4 => write!(f, "5:4"),
            Self::Num9_16 => write!(f, "9:16"),
            Self::Num16_9 => write!(f, "16:9"),
            Self::Num21_9 => write!(f, "21:9"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
