pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranslationRealtimeOutputFormat {
    Mp3_22050_32,
    Mp3_44100_32,
    Mp3_44100_64,
    Mp3_44100_96,
    Mp3_44100_128,
    Mp3_44100_192,
    Pcm16000,
    Pcm22050,
    Pcm24000,
    Pcm44100,
    Ulaw8000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranslationRealtimeOutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp3_22050_32 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp3_44100_32 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp3_44100_64 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp3_44100_96 => serializer.serialize_str("mp3_44100_96"),
            Self::Mp3_44100_128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp3_44100_192 => serializer.serialize_str("mp3_44100_192"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm22050 => serializer.serialize_str("pcm_22050"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Ulaw8000 => serializer.serialize_str("ulaw_8000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranslationRealtimeOutputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp3_22050_32" => Ok(Self::Mp3_22050_32),
            "mp3_44100_32" => Ok(Self::Mp3_44100_32),
            "mp3_44100_64" => Ok(Self::Mp3_44100_64),
            "mp3_44100_96" => Ok(Self::Mp3_44100_96),
            "mp3_44100_128" => Ok(Self::Mp3_44100_128),
            "mp3_44100_192" => Ok(Self::Mp3_44100_192),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_22050" => Ok(Self::Pcm22050),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_44100" => Ok(Self::Pcm44100),
            "ulaw_8000" => Ok(Self::Ulaw8000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranslationRealtimeOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp3_22050_32 => write!(f, "mp3_22050_32"),
            Self::Mp3_44100_32 => write!(f, "mp3_44100_32"),
            Self::Mp3_44100_64 => write!(f, "mp3_44100_64"),
            Self::Mp3_44100_96 => write!(f, "mp3_44100_96"),
            Self::Mp3_44100_128 => write!(f, "mp3_44100_128"),
            Self::Mp3_44100_192 => write!(f, "mp3_44100_192"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm22050 => write!(f, "pcm_22050"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Ulaw8000 => write!(f, "ulaw_8000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
