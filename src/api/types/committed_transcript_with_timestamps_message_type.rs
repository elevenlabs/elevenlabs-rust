pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommittedTranscriptWithTimestampsMessageType {
    CommittedTranscriptWithTimestamps,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CommittedTranscriptWithTimestampsMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CommittedTranscriptWithTimestamps => {
                serializer.serialize_str("committed_transcript_with_timestamps")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CommittedTranscriptWithTimestampsMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "committed_transcript_with_timestamps" => Ok(Self::CommittedTranscriptWithTimestamps),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CommittedTranscriptWithTimestampsMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommittedTranscriptWithTimestamps => {
                write!(f, "committed_transcript_with_timestamps")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
