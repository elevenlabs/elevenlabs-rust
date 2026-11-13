pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Veo3_1FastRequestDurationSecs {
    #[default]
    Num4,
    Num6,
    Num8,
}

impl Serialize for Veo3_1FastRequestDurationSecs {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num4 => serializer.serialize_i64(4),
            Self::Num6 => serializer.serialize_i64(6),
            Self::Num8 => serializer.serialize_i64(8),
        }
    }
}

impl<'de> Deserialize<'de> for Veo3_1FastRequestDurationSecs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if value == 4 {
            return Ok(Self::Num4);
        }
        if value == 6 {
            return Ok(Self::Num6);
        }
        if value == 8 {
            return Ok(Self::Num8);
        }
        Err(serde::de::Error::custom(format!(
            "unknown Veo3_1FastRequestDurationSecs value: {value}"
        )))
    }
}

impl fmt::Display for Veo3_1FastRequestDurationSecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num4 => write!(f, "4"),
            Self::Num6 => write!(f, "6"),
            Self::Num8 => write!(f, "8"),
        }
    }
}
