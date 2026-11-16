pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomGuardrailConfigModel {
    Gemini2_5FlashLite,
    Gemini2_5Flash,
    Gemini3_1FlashLite,
    Gemini3_5Flash,
    ClaudeHaiku4_5,
    ClaudeSonnet4_6,
    Gpt5_4Nano,
    Gpt5_4Mini,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CustomGuardrailConfigModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Gemini2_5FlashLite => serializer.serialize_str("gemini-2.5-flash-lite"),
            Self::Gemini2_5Flash => serializer.serialize_str("gemini-2.5-flash"),
            Self::Gemini3_1FlashLite => serializer.serialize_str("gemini-3.1-flash-lite"),
            Self::Gemini3_5Flash => serializer.serialize_str("gemini-3.5-flash"),
            Self::ClaudeHaiku4_5 => serializer.serialize_str("claude-haiku-4-5"),
            Self::ClaudeSonnet4_6 => serializer.serialize_str("claude-sonnet-4-6"),
            Self::Gpt5_4Nano => serializer.serialize_str("gpt-5.4-nano"),
            Self::Gpt5_4Mini => serializer.serialize_str("gpt-5.4-mini"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CustomGuardrailConfigModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "gemini-2.5-flash-lite" => Ok(Self::Gemini2_5FlashLite),
            "gemini-2.5-flash" => Ok(Self::Gemini2_5Flash),
            "gemini-3.1-flash-lite" => Ok(Self::Gemini3_1FlashLite),
            "gemini-3.5-flash" => Ok(Self::Gemini3_5Flash),
            "claude-haiku-4-5" => Ok(Self::ClaudeHaiku4_5),
            "claude-sonnet-4-6" => Ok(Self::ClaudeSonnet4_6),
            "gpt-5.4-nano" => Ok(Self::Gpt5_4Nano),
            "gpt-5.4-mini" => Ok(Self::Gpt5_4Mini),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CustomGuardrailConfigModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gemini2_5FlashLite => write!(f, "gemini-2.5-flash-lite"),
            Self::Gemini2_5Flash => write!(f, "gemini-2.5-flash"),
            Self::Gemini3_1FlashLite => write!(f, "gemini-3.1-flash-lite"),
            Self::Gemini3_5Flash => write!(f, "gemini-3.5-flash"),
            Self::ClaudeHaiku4_5 => write!(f, "claude-haiku-4-5"),
            Self::ClaudeSonnet4_6 => write!(f, "claude-sonnet-4-6"),
            Self::Gpt5_4Nano => write!(f, "gpt-5.4-nano"),
            Self::Gpt5_4Mini => write!(f, "gpt-5.4-mini"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
