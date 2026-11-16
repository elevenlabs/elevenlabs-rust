pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElevenFlashV2_5RequestOutputFormat {
    Mp3_22050_32,
    Mp3_24000_48,
    Mp3_44100_32,
    Mp3_44100_64,
    Mp3_44100_96,
    Mp3_44100_128,
    Mp3_44100_192,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ElevenFlashV2_5RequestOutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp3_22050_32 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp3_24000_48 => serializer.serialize_str("mp3_24000_48"),
            Self::Mp3_44100_32 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp3_44100_64 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp3_44100_96 => serializer.serialize_str("mp3_44100_96"),
            Self::Mp3_44100_128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp3_44100_192 => serializer.serialize_str("mp3_44100_192"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ElevenFlashV2_5RequestOutputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp3_22050_32" => Ok(Self::Mp3_22050_32),
            "mp3_24000_48" => Ok(Self::Mp3_24000_48),
            "mp3_44100_32" => Ok(Self::Mp3_44100_32),
            "mp3_44100_64" => Ok(Self::Mp3_44100_64),
            "mp3_44100_96" => Ok(Self::Mp3_44100_96),
            "mp3_44100_128" => Ok(Self::Mp3_44100_128),
            "mp3_44100_192" => Ok(Self::Mp3_44100_192),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ElevenFlashV2_5RequestOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp3_22050_32 => write!(f, "mp3_22050_32"),
            Self::Mp3_24000_48 => write!(f, "mp3_24000_48"),
            Self::Mp3_44100_32 => write!(f, "mp3_44100_32"),
            Self::Mp3_44100_64 => write!(f, "mp3_44100_64"),
            Self::Mp3_44100_96 => write!(f, "mp3_44100_96"),
            Self::Mp3_44100_128 => write!(f, "mp3_44100_128"),
            Self::Mp3_44100_192 => write!(f, "mp3_44100_192"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
