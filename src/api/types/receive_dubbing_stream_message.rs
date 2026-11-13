pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "message_type")]
#[non_exhaustive]
pub enum ReceiveDubbingStreamMessage {
    #[serde(rename = "session_started")]
    SessionStarted {
        #[serde(flatten)]
        data: DubbingSessionStarted,
    },

    #[serde(rename = "status")]
    Status {
        #[serde(flatten)]
        data: ReceiveDubbingStreamMessage2,
    },

    #[serde(rename = "partial_transcript")]
    PartialTranscript {
        #[serde(flatten)]
        data: DubbingPartialTranscript,
    },

    #[serde(rename = "committed_transcript")]
    CommittedTranscript {
        #[serde(flatten)]
        data: DubbingCommittedTranscript,
    },

    #[serde(rename = "translation")]
    Translation {
        #[serde(flatten)]
        data: DubbingTranslation,
    },

    #[serde(rename = "audio")]
    Audio {
        #[serde(flatten)]
        data: DubbingAudio,
    },

    #[serde(rename = "error")]
    Error {
        #[serde(flatten)]
        data: DubbingErrorPayload,
    },

    #[serde(rename = "auth_error")]
    AuthError {
        #[serde(flatten)]
        data: DubbingAuthError,
    },

    #[serde(rename = "rate_limited")]
    RateLimited {
        #[serde(flatten)]
        data: DubbingRateLimitedError,
    },

    #[serde(rename = "queue_overflow")]
    QueueOverflow {
        #[serde(flatten)]
        data: DubbingQueueOverflowError,
    },

    #[serde(rename = "invalid_request")]
    InvalidRequest {
        #[serde(flatten)]
        data: DubbingInvalidRequestError,
    },

    #[serde(rename = "input_error")]
    InputError {
        #[serde(flatten)]
        data: DubbingInputError,
    },

    #[serde(rename = "transcriber_error")]
    TranscriberError {
        #[serde(flatten)]
        data: DubbingTranscriberError,
    },

    #[serde(rename = "translation_error")]
    TranslationError {
        #[serde(flatten)]
        data: DubbingTranslationError,
    },

    #[serde(rename = "audio_output_error")]
    AudioOutputError {
        #[serde(flatten)]
        data: DubbingAudioOutputError,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ReceiveDubbingStreamMessage {
    pub fn is_session_started(&self) -> bool {
        matches!(self, Self::SessionStarted { .. })
    }

    pub fn as_session_started(&self) -> Option<&DubbingSessionStarted> {
        match self {
            Self::SessionStarted { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_session_started(self) -> Option<DubbingSessionStarted> {
        match self {
            Self::SessionStarted { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status { .. })
    }

    pub fn as_status(&self) -> Option<&ReceiveDubbingStreamMessage2> {
        match self {
            Self::Status { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_status(self) -> Option<ReceiveDubbingStreamMessage2> {
        match self {
            Self::Status { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_partial_transcript(&self) -> bool {
        matches!(self, Self::PartialTranscript { .. })
    }

    pub fn as_partial_transcript(&self) -> Option<&DubbingPartialTranscript> {
        match self {
            Self::PartialTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_partial_transcript(self) -> Option<DubbingPartialTranscript> {
        match self {
            Self::PartialTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_committed_transcript(&self) -> bool {
        matches!(self, Self::CommittedTranscript { .. })
    }

    pub fn as_committed_transcript(&self) -> Option<&DubbingCommittedTranscript> {
        match self {
            Self::CommittedTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_committed_transcript(self) -> Option<DubbingCommittedTranscript> {
        match self {
            Self::CommittedTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_translation(&self) -> bool {
        matches!(self, Self::Translation { .. })
    }

    pub fn as_translation(&self) -> Option<&DubbingTranslation> {
        match self {
            Self::Translation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_translation(self) -> Option<DubbingTranslation> {
        match self {
            Self::Translation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. })
    }

    pub fn as_audio(&self) -> Option<&DubbingAudio> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio(self) -> Option<DubbingAudio> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn as_error(&self) -> Option<&DubbingErrorPayload> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<DubbingErrorPayload> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError { .. })
    }

    pub fn as_auth_error(&self) -> Option<&DubbingAuthError> {
        match self {
            Self::AuthError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_auth_error(self) -> Option<DubbingAuthError> {
        match self {
            Self::AuthError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited { .. })
    }

    pub fn as_rate_limited(&self) -> Option<&DubbingRateLimitedError> {
        match self {
            Self::RateLimited { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_rate_limited(self) -> Option<DubbingRateLimitedError> {
        match self {
            Self::RateLimited { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_queue_overflow(&self) -> bool {
        matches!(self, Self::QueueOverflow { .. })
    }

    pub fn as_queue_overflow(&self) -> Option<&DubbingQueueOverflowError> {
        match self {
            Self::QueueOverflow { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_queue_overflow(self) -> Option<DubbingQueueOverflowError> {
        match self {
            Self::QueueOverflow { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_invalid_request(&self) -> bool {
        matches!(self, Self::InvalidRequest { .. })
    }

    pub fn as_invalid_request(&self) -> Option<&DubbingInvalidRequestError> {
        match self {
            Self::InvalidRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_invalid_request(self) -> Option<DubbingInvalidRequestError> {
        match self {
            Self::InvalidRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_input_error(&self) -> bool {
        matches!(self, Self::InputError { .. })
    }

    pub fn as_input_error(&self) -> Option<&DubbingInputError> {
        match self {
            Self::InputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_input_error(self) -> Option<DubbingInputError> {
        match self {
            Self::InputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transcriber_error(&self) -> bool {
        matches!(self, Self::TranscriberError { .. })
    }

    pub fn as_transcriber_error(&self) -> Option<&DubbingTranscriberError> {
        match self {
            Self::TranscriberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transcriber_error(self) -> Option<DubbingTranscriberError> {
        match self {
            Self::TranscriberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_translation_error(&self) -> bool {
        matches!(self, Self::TranslationError { .. })
    }

    pub fn as_translation_error(&self) -> Option<&DubbingTranslationError> {
        match self {
            Self::TranslationError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_translation_error(self) -> Option<DubbingTranslationError> {
        match self {
            Self::TranslationError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio_output_error(&self) -> bool {
        matches!(self, Self::AudioOutputError { .. })
    }

    pub fn as_audio_output_error(&self) -> Option<&DubbingAudioOutputError> {
        match self {
            Self::AudioOutputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio_output_error(self) -> Option<DubbingAudioOutputError> {
        match self {
            Self::AudioOutputError { data } => Some(data),
            _ => None,
        }
    }
}
