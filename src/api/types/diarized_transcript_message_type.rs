pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiarizedTranscriptMessageType {
    DiarizedTranscript,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DiarizedTranscriptMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DiarizedTranscript => serializer.serialize_str("diarized_transcript"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DiarizedTranscriptMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "diarized_transcript" => Ok(Self::DiarizedTranscript),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DiarizedTranscriptMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DiarizedTranscript => write!(f, "diarized_transcript"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
