pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConvertOutputFormatQueryParameter1 {
    Alaw8000,
    Mp3_22050_32,
    Mp3_24000_48,
    Mp3_44100_128,
    Mp3_44100_192,
    Mp3_44100_32,
    Mp3_44100_64,
    Mp3_44100_96,
    Opus48000_128,
    Opus48000_192,
    Opus48000_32,
    Opus48000_64,
    Opus48000_96,
    Pcm16000,
    Pcm22050,
    Pcm24000,
    Pcm32000,
    Pcm44100,
    Pcm48000,
    Pcm8000,
    Ulaw8000,
    Wav16000,
    Wav22050,
    Wav24000,
    Wav32000,
    Wav44100,
    Wav48000,
    Wav8000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConvertOutputFormatQueryParameter1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Alaw8000 => serializer.serialize_str("alaw_8000"),
            Self::Mp3_22050_32 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp3_24000_48 => serializer.serialize_str("mp3_24000_48"),
            Self::Mp3_44100_128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp3_44100_192 => serializer.serialize_str("mp3_44100_192"),
            Self::Mp3_44100_32 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp3_44100_64 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp3_44100_96 => serializer.serialize_str("mp3_44100_96"),
            Self::Opus48000_128 => serializer.serialize_str("opus_48000_128"),
            Self::Opus48000_192 => serializer.serialize_str("opus_48000_192"),
            Self::Opus48000_32 => serializer.serialize_str("opus_48000_32"),
            Self::Opus48000_64 => serializer.serialize_str("opus_48000_64"),
            Self::Opus48000_96 => serializer.serialize_str("opus_48000_96"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm22050 => serializer.serialize_str("pcm_22050"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm32000 => serializer.serialize_str("pcm_32000"),
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Pcm48000 => serializer.serialize_str("pcm_48000"),
            Self::Pcm8000 => serializer.serialize_str("pcm_8000"),
            Self::Ulaw8000 => serializer.serialize_str("ulaw_8000"),
            Self::Wav16000 => serializer.serialize_str("wav_16000"),
            Self::Wav22050 => serializer.serialize_str("wav_22050"),
            Self::Wav24000 => serializer.serialize_str("wav_24000"),
            Self::Wav32000 => serializer.serialize_str("wav_32000"),
            Self::Wav44100 => serializer.serialize_str("wav_44100"),
            Self::Wav48000 => serializer.serialize_str("wav_48000"),
            Self::Wav8000 => serializer.serialize_str("wav_8000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConvertOutputFormatQueryParameter1 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "alaw_8000" => Ok(Self::Alaw8000),
            "mp3_22050_32" => Ok(Self::Mp3_22050_32),
            "mp3_24000_48" => Ok(Self::Mp3_24000_48),
            "mp3_44100_128" => Ok(Self::Mp3_44100_128),
            "mp3_44100_192" => Ok(Self::Mp3_44100_192),
            "mp3_44100_32" => Ok(Self::Mp3_44100_32),
            "mp3_44100_64" => Ok(Self::Mp3_44100_64),
            "mp3_44100_96" => Ok(Self::Mp3_44100_96),
            "opus_48000_128" => Ok(Self::Opus48000_128),
            "opus_48000_192" => Ok(Self::Opus48000_192),
            "opus_48000_32" => Ok(Self::Opus48000_32),
            "opus_48000_64" => Ok(Self::Opus48000_64),
            "opus_48000_96" => Ok(Self::Opus48000_96),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_22050" => Ok(Self::Pcm22050),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_32000" => Ok(Self::Pcm32000),
            "pcm_44100" => Ok(Self::Pcm44100),
            "pcm_48000" => Ok(Self::Pcm48000),
            "pcm_8000" => Ok(Self::Pcm8000),
            "ulaw_8000" => Ok(Self::Ulaw8000),
            "wav_16000" => Ok(Self::Wav16000),
            "wav_22050" => Ok(Self::Wav22050),
            "wav_24000" => Ok(Self::Wav24000),
            "wav_32000" => Ok(Self::Wav32000),
            "wav_44100" => Ok(Self::Wav44100),
            "wav_48000" => Ok(Self::Wav48000),
            "wav_8000" => Ok(Self::Wav8000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConvertOutputFormatQueryParameter1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alaw8000 => write!(f, "alaw_8000"),
            Self::Mp3_22050_32 => write!(f, "mp3_22050_32"),
            Self::Mp3_24000_48 => write!(f, "mp3_24000_48"),
            Self::Mp3_44100_128 => write!(f, "mp3_44100_128"),
            Self::Mp3_44100_192 => write!(f, "mp3_44100_192"),
            Self::Mp3_44100_32 => write!(f, "mp3_44100_32"),
            Self::Mp3_44100_64 => write!(f, "mp3_44100_64"),
            Self::Mp3_44100_96 => write!(f, "mp3_44100_96"),
            Self::Opus48000_128 => write!(f, "opus_48000_128"),
            Self::Opus48000_192 => write!(f, "opus_48000_192"),
            Self::Opus48000_32 => write!(f, "opus_48000_32"),
            Self::Opus48000_64 => write!(f, "opus_48000_64"),
            Self::Opus48000_96 => write!(f, "opus_48000_96"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm22050 => write!(f, "pcm_22050"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm32000 => write!(f, "pcm_32000"),
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Pcm48000 => write!(f, "pcm_48000"),
            Self::Pcm8000 => write!(f, "pcm_8000"),
            Self::Ulaw8000 => write!(f, "ulaw_8000"),
            Self::Wav16000 => write!(f, "wav_16000"),
            Self::Wav22050 => write!(f, "wav_22050"),
            Self::Wav24000 => write!(f, "wav_24000"),
            Self::Wav32000 => write!(f, "wav_32000"),
            Self::Wav44100 => write!(f, "wav_44100"),
            Self::Wav48000 => write!(f, "wav_48000"),
            Self::Wav8000 => write!(f, "wav_8000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
