pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TtsOptimizeStreamingLatency {
    #[default]
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
}

impl Serialize for TtsOptimizeStreamingLatency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num0 => serializer.serialize_i64(0),
            Self::Num1 => serializer.serialize_i64(1),
            Self::Num2 => serializer.serialize_i64(2),
            Self::Num3 => serializer.serialize_i64(3),
            Self::Num4 => serializer.serialize_i64(4),
        }
    }
}

impl<'de> Deserialize<'de> for TtsOptimizeStreamingLatency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if value == 0 {
            return Ok(Self::Num0);
        }
        if value == 1 {
            return Ok(Self::Num1);
        }
        if value == 2 {
            return Ok(Self::Num2);
        }
        if value == 3 {
            return Ok(Self::Num3);
        }
        if value == 4 {
            return Ok(Self::Num4);
        }
        Err(serde::de::Error::custom(format!(
            "unknown TtsOptimizeStreamingLatency value: {value}"
        )))
    }
}

impl fmt::Display for TtsOptimizeStreamingLatency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num0 => write!(f, "0"),
            Self::Num1 => write!(f, "1"),
            Self::Num2 => write!(f, "2"),
            Self::Num3 => write!(f, "3"),
            Self::Num4 => write!(f, "4"),
        }
    }
}
