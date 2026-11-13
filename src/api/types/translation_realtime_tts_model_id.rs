pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranslationRealtimeTtsModelId {
    ElevenFlashV2_5,
    ElevenV3,
    ElevenV4Turbo,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranslationRealtimeTtsModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ElevenFlashV2_5 => serializer.serialize_str("eleven_flash_v2_5"),
            Self::ElevenV3 => serializer.serialize_str("eleven_v3"),
            Self::ElevenV4Turbo => serializer.serialize_str("eleven_v4_turbo"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranslationRealtimeTtsModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eleven_flash_v2_5" => Ok(Self::ElevenFlashV2_5),
            "eleven_v3" => Ok(Self::ElevenV3),
            "eleven_v4_turbo" => Ok(Self::ElevenV4Turbo),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranslationRealtimeTtsModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenFlashV2_5 => write!(f, "eleven_flash_v2_5"),
            Self::ElevenV3 => write!(f, "eleven_v3"),
            Self::ElevenV4Turbo => write!(f, "eleven_v4_turbo"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
