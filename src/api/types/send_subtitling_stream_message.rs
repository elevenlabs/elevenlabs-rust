pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "message_type")]
#[non_exhaustive]
pub enum SendSubtitlingStreamMessage {
    #[serde(rename = "input_audio_chunk")]
    InputAudioChunk {
        #[serde(flatten)]
        data: SubtitlingInputAudioChunk,
    },

    #[serde(rename = "end_of_stream")]
    EndOfStream {
        #[serde(flatten)]
        data: SubtitlingEndOfStream,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SendSubtitlingStreamMessage {
    pub fn is_input_audio_chunk(&self) -> bool {
        matches!(self, Self::InputAudioChunk { .. })
    }

    pub fn as_input_audio_chunk(&self) -> Option<&SubtitlingInputAudioChunk> {
        match self {
            Self::InputAudioChunk { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_input_audio_chunk(self) -> Option<SubtitlingInputAudioChunk> {
        match self {
            Self::InputAudioChunk { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_end_of_stream(&self) -> bool {
        matches!(self, Self::EndOfStream { .. })
    }

    pub fn as_end_of_stream(&self) -> Option<&SubtitlingEndOfStream> {
        match self {
            Self::EndOfStream { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end_of_stream(self) -> Option<SubtitlingEndOfStream> {
        match self {
            Self::EndOfStream { data } => Some(data),
            _ => None,
        }
    }
}
