pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "message_type")]
#[non_exhaustive]
pub enum ReceiveTranslateStreamMessage {
    #[serde(rename = "session_started")]
    SessionStarted {
        #[serde(flatten)]
        data: TranslateSessionStarted,
    },

    #[serde(rename = "status")]
    Status {
        #[serde(flatten)]
        data: TranslateStatus,
    },

    #[serde(rename = "partial_transcript")]
    PartialTranscript {
        #[serde(flatten)]
        data: TranslatePartialTranscript,
    },

    #[serde(rename = "final_transcript")]
    FinalTranscript {
        #[serde(flatten)]
        data: TranslateFinalTranscript,
    },

    #[serde(rename = "translation")]
    Translation {
        #[serde(flatten)]
        data: TranslateTranslation,
    },

    #[serde(rename = "audio")]
    Audio {
        #[serde(flatten)]
        data: TranslateAudio,
    },

    #[serde(rename = "error")]
    Error {
        #[serde(flatten)]
        data: TranslateError,
    },

    #[serde(rename = "auth_error")]
    AuthError {
        #[serde(flatten)]
        data: TranslateAuthError,
    },

    #[serde(rename = "rate_limited")]
    RateLimited {
        #[serde(flatten)]
        data: TranslateRateLimitedError,
    },

    #[serde(rename = "queue_overflow")]
    QueueOverflow {
        #[serde(flatten)]
        data: TranslateQueueOverflowError,
    },

    #[serde(rename = "input_error")]
    InputError {
        #[serde(flatten)]
        data: TranslateInputError,
    },

    #[serde(rename = "transcriber_error")]
    TranscriberError {
        #[serde(flatten)]
        data: TranslateTranscriberError,
    },

    #[serde(rename = "translation_error")]
    TranslationError {
        #[serde(flatten)]
        data: TranslateTranslationError,
    },

    #[serde(rename = "audio_output_error")]
    AudioOutputError {
        #[serde(flatten)]
        data: TranslateAudioOutputError,
    },

    #[serde(rename = "quota_exceeded")]
    QuotaExceeded {
        #[serde(flatten)]
        data: TranslateQuotaExceededError,
    },

    #[serde(rename = "payment_issue")]
    PaymentIssue {
        #[serde(flatten)]
        data: TranslatePaymentIssueError,
    },

    #[serde(rename = "unaccepted_terms")]
    UnacceptedTerms {
        #[serde(flatten)]
        data: TranslateUnacceptedTermsError,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ReceiveTranslateStreamMessage {
    pub fn is_session_started(&self) -> bool {
        matches!(self, Self::SessionStarted { .. })
    }

    pub fn as_session_started(&self) -> Option<&TranslateSessionStarted> {
        match self {
            Self::SessionStarted { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_session_started(self) -> Option<TranslateSessionStarted> {
        match self {
            Self::SessionStarted { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status { .. })
    }

    pub fn as_status(&self) -> Option<&TranslateStatus> {
        match self {
            Self::Status { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_status(self) -> Option<TranslateStatus> {
        match self {
            Self::Status { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_partial_transcript(&self) -> bool {
        matches!(self, Self::PartialTranscript { .. })
    }

    pub fn as_partial_transcript(&self) -> Option<&TranslatePartialTranscript> {
        match self {
            Self::PartialTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_partial_transcript(self) -> Option<TranslatePartialTranscript> {
        match self {
            Self::PartialTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_final_transcript(&self) -> bool {
        matches!(self, Self::FinalTranscript { .. })
    }

    pub fn as_final_transcript(&self) -> Option<&TranslateFinalTranscript> {
        match self {
            Self::FinalTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_final_transcript(self) -> Option<TranslateFinalTranscript> {
        match self {
            Self::FinalTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_translation(&self) -> bool {
        matches!(self, Self::Translation { .. })
    }

    pub fn as_translation(&self) -> Option<&TranslateTranslation> {
        match self {
            Self::Translation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_translation(self) -> Option<TranslateTranslation> {
        match self {
            Self::Translation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. })
    }

    pub fn as_audio(&self) -> Option<&TranslateAudio> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio(self) -> Option<TranslateAudio> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn as_error(&self) -> Option<&TranslateError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<TranslateError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError { .. })
    }

    pub fn as_auth_error(&self) -> Option<&TranslateAuthError> {
        match self {
            Self::AuthError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_auth_error(self) -> Option<TranslateAuthError> {
        match self {
            Self::AuthError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited { .. })
    }

    pub fn as_rate_limited(&self) -> Option<&TranslateRateLimitedError> {
        match self {
            Self::RateLimited { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_rate_limited(self) -> Option<TranslateRateLimitedError> {
        match self {
            Self::RateLimited { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_queue_overflow(&self) -> bool {
        matches!(self, Self::QueueOverflow { .. })
    }

    pub fn as_queue_overflow(&self) -> Option<&TranslateQueueOverflowError> {
        match self {
            Self::QueueOverflow { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_queue_overflow(self) -> Option<TranslateQueueOverflowError> {
        match self {
            Self::QueueOverflow { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_input_error(&self) -> bool {
        matches!(self, Self::InputError { .. })
    }

    pub fn as_input_error(&self) -> Option<&TranslateInputError> {
        match self {
            Self::InputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_input_error(self) -> Option<TranslateInputError> {
        match self {
            Self::InputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transcriber_error(&self) -> bool {
        matches!(self, Self::TranscriberError { .. })
    }

    pub fn as_transcriber_error(&self) -> Option<&TranslateTranscriberError> {
        match self {
            Self::TranscriberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transcriber_error(self) -> Option<TranslateTranscriberError> {
        match self {
            Self::TranscriberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_translation_error(&self) -> bool {
        matches!(self, Self::TranslationError { .. })
    }

    pub fn as_translation_error(&self) -> Option<&TranslateTranslationError> {
        match self {
            Self::TranslationError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_translation_error(self) -> Option<TranslateTranslationError> {
        match self {
            Self::TranslationError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio_output_error(&self) -> bool {
        matches!(self, Self::AudioOutputError { .. })
    }

    pub fn as_audio_output_error(&self) -> Option<&TranslateAudioOutputError> {
        match self {
            Self::AudioOutputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio_output_error(self) -> Option<TranslateAudioOutputError> {
        match self {
            Self::AudioOutputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_quota_exceeded(&self) -> bool {
        matches!(self, Self::QuotaExceeded { .. })
    }

    pub fn as_quota_exceeded(&self) -> Option<&TranslateQuotaExceededError> {
        match self {
            Self::QuotaExceeded { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_quota_exceeded(self) -> Option<TranslateQuotaExceededError> {
        match self {
            Self::QuotaExceeded { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_payment_issue(&self) -> bool {
        matches!(self, Self::PaymentIssue { .. })
    }

    pub fn as_payment_issue(&self) -> Option<&TranslatePaymentIssueError> {
        match self {
            Self::PaymentIssue { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_payment_issue(self) -> Option<TranslatePaymentIssueError> {
        match self {
            Self::PaymentIssue { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_unaccepted_terms(&self) -> bool {
        matches!(self, Self::UnacceptedTerms { .. })
    }

    pub fn as_unaccepted_terms(&self) -> Option<&TranslateUnacceptedTermsError> {
        match self {
            Self::UnacceptedTerms { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_unaccepted_terms(self) -> Option<TranslateUnacceptedTermsError> {
        match self {
            Self::UnacceptedTerms { data } => Some(data),
            _ => None,
        }
    }
}
