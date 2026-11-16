pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StreamOutputFormatQueryParameter1 {
    Mp3_22050_32,
    Mp3_24000_48,
    Mp3_44100_32,
    Mp3_44100_64,
    Mp3_44100_96,
    Mp3_44100_128,
    Mp3_44100_192,
    Pcm8000,
    Pcm16000,
    Pcm22050,
    Pcm24000,
    Pcm32000,
    Pcm44100,
    Pcm48000,
    Ulaw8000,
    Alaw8000,
    Opus48000_32,
    Opus48000_64,
    Opus48000_96,
    Opus48000_128,
    Opus48000_192,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StreamOutputFormatQueryParameter1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp3_22050_32 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp3_24000_48 => serializer.serialize_str("mp3_24000_48"),
            Self::Mp3_44100_32 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp3_44100_64 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp3_44100_96 => serializer.serialize_str("mp3_44100_96"),
            Self::Mp3_44100_128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp3_44100_192 => serializer.serialize_str("mp3_44100_192"),
            Self::Pcm8000 => serializer.serialize_str("pcm_8000"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm22050 => serializer.serialize_str("pcm_22050"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm32000 => serializer.serialize_str("pcm_32000"),
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Pcm48000 => serializer.serialize_str("pcm_48000"),
            Self::Ulaw8000 => serializer.serialize_str("ulaw_8000"),
            Self::Alaw8000 => serializer.serialize_str("alaw_8000"),
            Self::Opus48000_32 => serializer.serialize_str("opus_48000_32"),
            Self::Opus48000_64 => serializer.serialize_str("opus_48000_64"),
            Self::Opus48000_96 => serializer.serialize_str("opus_48000_96"),
            Self::Opus48000_128 => serializer.serialize_str("opus_48000_128"),
            Self::Opus48000_192 => serializer.serialize_str("opus_48000_192"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StreamOutputFormatQueryParameter1 {
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
            "pcm_8000" => Ok(Self::Pcm8000),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_22050" => Ok(Self::Pcm22050),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_32000" => Ok(Self::Pcm32000),
            "pcm_44100" => Ok(Self::Pcm44100),
            "pcm_48000" => Ok(Self::Pcm48000),
            "ulaw_8000" => Ok(Self::Ulaw8000),
            "alaw_8000" => Ok(Self::Alaw8000),
            "opus_48000_32" => Ok(Self::Opus48000_32),
            "opus_48000_64" => Ok(Self::Opus48000_64),
            "opus_48000_96" => Ok(Self::Opus48000_96),
            "opus_48000_128" => Ok(Self::Opus48000_128),
            "opus_48000_192" => Ok(Self::Opus48000_192),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StreamOutputFormatQueryParameter1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp3_22050_32 => write!(f, "mp3_22050_32"),
            Self::Mp3_24000_48 => write!(f, "mp3_24000_48"),
            Self::Mp3_44100_32 => write!(f, "mp3_44100_32"),
            Self::Mp3_44100_64 => write!(f, "mp3_44100_64"),
            Self::Mp3_44100_96 => write!(f, "mp3_44100_96"),
            Self::Mp3_44100_128 => write!(f, "mp3_44100_128"),
            Self::Mp3_44100_192 => write!(f, "mp3_44100_192"),
            Self::Pcm8000 => write!(f, "pcm_8000"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm22050 => write!(f, "pcm_22050"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm32000 => write!(f, "pcm_32000"),
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Pcm48000 => write!(f, "pcm_48000"),
            Self::Ulaw8000 => write!(f, "ulaw_8000"),
            Self::Alaw8000 => write!(f, "alaw_8000"),
            Self::Opus48000_32 => write!(f, "opus_48000_32"),
            Self::Opus48000_64 => write!(f, "opus_48000_64"),
            Self::Opus48000_96 => write!(f, "opus_48000_96"),
            Self::Opus48000_128 => write!(f, "opus_48000_128"),
            Self::Opus48000_192 => write!(f, "opus_48000_192"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
