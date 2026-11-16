pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChapterContentBlockTtsNodeResponseModelType {
    TtsNode,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChapterContentBlockTtsNodeResponseModelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TtsNode => serializer.serialize_str("tts_node"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChapterContentBlockTtsNodeResponseModelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tts_node" => Ok(Self::TtsNode),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChapterContentBlockTtsNodeResponseModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TtsNode => write!(f, "tts_node"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
