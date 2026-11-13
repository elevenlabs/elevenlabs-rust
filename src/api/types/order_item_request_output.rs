pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum OrderItemRequestOutput {
    #[serde(rename = "dub")]
    Dub {
        #[serde(flatten)]
        data: DubOrderItemRequest,
    },

    #[serde(rename = "subtitles")]
    Subtitles {
        #[serde(flatten)]
        data: SubtitleOrderItemRequest,
    },

    #[serde(rename = "transcription")]
    Transcription {
        #[serde(flatten)]
        data: TranscriptionOrderItemRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OrderItemRequestOutput {
    pub fn is_dub(&self) -> bool {
        matches!(self, Self::Dub { .. })
    }

    pub fn as_dub(&self) -> Option<&DubOrderItemRequest> {
        match self {
            Self::Dub { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_dub(self) -> Option<DubOrderItemRequest> {
        match self {
            Self::Dub { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_subtitles(&self) -> bool {
        matches!(self, Self::Subtitles { .. })
    }

    pub fn as_subtitles(&self) -> Option<&SubtitleOrderItemRequest> {
        match self {
            Self::Subtitles { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_subtitles(self) -> Option<SubtitleOrderItemRequest> {
        match self {
            Self::Subtitles { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transcription(&self) -> bool {
        matches!(self, Self::Transcription { .. })
    }

    pub fn as_transcription(&self) -> Option<&TranscriptionOrderItemRequest> {
        match self {
            Self::Transcription { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transcription(self) -> Option<TranscriptionOrderItemRequest> {
        match self {
            Self::Transcription { data } => Some(data),
            _ => None,
        }
    }
}
