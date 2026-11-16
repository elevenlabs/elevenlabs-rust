pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum SpeechToTextChunkResponseModelEditedTranscript {
    #[serde(rename = "error")]
    Error {
        #[serde(flatten)]
        data: TranscriptEditError,
    },

    #[serde(rename = "transcript")]
    Transcript {
        #[serde(flatten)]
        data: EditedTranscriptResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SpeechToTextChunkResponseModelEditedTranscript {
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn as_error(&self) -> Option<&TranscriptEditError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<TranscriptEditError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transcript(&self) -> bool {
        matches!(self, Self::Transcript { .. })
    }

    pub fn as_transcript(&self) -> Option<&EditedTranscriptResponse> {
        match self {
            Self::Transcript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transcript(self) -> Option<EditedTranscriptResponse> {
        match self {
            Self::Transcript { data } => Some(data),
            _ => None,
        }
    }
}
