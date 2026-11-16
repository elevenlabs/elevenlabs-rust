pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DubbingLiveInputNumChannels {
    #[default]
    Num1,
    Num2,
}

impl Serialize for DubbingLiveInputNumChannels {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num1 => serializer.serialize_i64(1),
            Self::Num2 => serializer.serialize_i64(2),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingLiveInputNumChannels {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if value == 1 {
            return Ok(Self::Num1);
        }
        if value == 2 {
            return Ok(Self::Num2);
        }
        Err(serde::de::Error::custom(format!(
            "unknown DubbingLiveInputNumChannels value: {value}"
        )))
    }
}

impl fmt::Display for DubbingLiveInputNumChannels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num1 => write!(f, "1"),
            Self::Num2 => write!(f, "2"),
        }
    }
}
