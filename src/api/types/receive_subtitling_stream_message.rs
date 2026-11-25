pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "message_type")]
#[non_exhaustive]
pub enum ReceiveSubtitlingStreamMessage {
    #[serde(rename = "session_started")]
    SessionStarted {
        #[serde(flatten)]
        data: SubtitlingSessionStarted,
    },

    #[serde(rename = "status")]
    Status {
        #[serde(flatten)]
        data: SubtitlingStatus,
    },

    #[serde(rename = "partial_transcript")]
    PartialTranscript {
        #[serde(flatten)]
        data: SubtitlingPartialTranscript,
    },

    #[serde(rename = "final_transcript")]
    FinalTranscript {
        #[serde(flatten)]
        data: SubtitlingFinalTranscript,
    },

    #[serde(rename = "partial_subtitle")]
    PartialSubtitle {
        #[serde(flatten)]
        data: SubtitlingPartialSubtitle,
    },

    #[serde(rename = "committed_subtitle")]
    CommittedSubtitle {
        #[serde(flatten)]
        data: SubtitlingCommittedSubtitle,
    },

    #[serde(rename = "error")]
    Error {
        #[serde(flatten)]
        data: SubtitlingError,
    },

    #[serde(rename = "auth_error")]
    AuthError {
        #[serde(flatten)]
        data: SubtitlingAuthError,
    },

    #[serde(rename = "rate_limited")]
    RateLimited {
        #[serde(flatten)]
        data: SubtitlingRateLimitedError,
    },

    #[serde(rename = "queue_overflow")]
    QueueOverflow {
        #[serde(flatten)]
        data: SubtitlingQueueOverflowError,
    },

    #[serde(rename = "input_error")]
    InputError {
        #[serde(flatten)]
        data: SubtitlingInputError,
    },

    #[serde(rename = "transcriber_error")]
    TranscriberError {
        #[serde(flatten)]
        data: SubtitlingTranscriberError,
    },

    #[serde(rename = "translation_error")]
    TranslationError {
        #[serde(flatten)]
        data: SubtitlingTranslationError,
    },

    #[serde(rename = "quota_exceeded")]
    QuotaExceeded {
        #[serde(flatten)]
        data: SubtitlingQuotaExceededError,
    },

    #[serde(rename = "payment_issue")]
    PaymentIssue {
        #[serde(flatten)]
        data: SubtitlingPaymentIssueError,
    },

    #[serde(rename = "unaccepted_terms")]
    UnacceptedTerms {
        #[serde(flatten)]
        data: SubtitlingUnacceptedTermsError,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ReceiveSubtitlingStreamMessage {
    pub fn is_session_started(&self) -> bool {
        matches!(self, Self::SessionStarted { .. })
    }

    pub fn as_session_started(&self) -> Option<&SubtitlingSessionStarted> {
        match self {
            Self::SessionStarted { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_session_started(self) -> Option<SubtitlingSessionStarted> {
        match self {
            Self::SessionStarted { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status { .. })
    }

    pub fn as_status(&self) -> Option<&SubtitlingStatus> {
        match self {
            Self::Status { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_status(self) -> Option<SubtitlingStatus> {
        match self {
            Self::Status { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_partial_transcript(&self) -> bool {
        matches!(self, Self::PartialTranscript { .. })
    }

    pub fn as_partial_transcript(&self) -> Option<&SubtitlingPartialTranscript> {
        match self {
            Self::PartialTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_partial_transcript(self) -> Option<SubtitlingPartialTranscript> {
        match self {
            Self::PartialTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_final_transcript(&self) -> bool {
        matches!(self, Self::FinalTranscript { .. })
    }

    pub fn as_final_transcript(&self) -> Option<&SubtitlingFinalTranscript> {
        match self {
            Self::FinalTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_final_transcript(self) -> Option<SubtitlingFinalTranscript> {
        match self {
            Self::FinalTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_partial_subtitle(&self) -> bool {
        matches!(self, Self::PartialSubtitle { .. })
    }

    pub fn as_partial_subtitle(&self) -> Option<&SubtitlingPartialSubtitle> {
        match self {
            Self::PartialSubtitle { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_partial_subtitle(self) -> Option<SubtitlingPartialSubtitle> {
        match self {
            Self::PartialSubtitle { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_committed_subtitle(&self) -> bool {
        matches!(self, Self::CommittedSubtitle { .. })
    }

    pub fn as_committed_subtitle(&self) -> Option<&SubtitlingCommittedSubtitle> {
        match self {
            Self::CommittedSubtitle { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_committed_subtitle(self) -> Option<SubtitlingCommittedSubtitle> {
        match self {
            Self::CommittedSubtitle { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn as_error(&self) -> Option<&SubtitlingError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<SubtitlingError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError { .. })
    }

    pub fn as_auth_error(&self) -> Option<&SubtitlingAuthError> {
        match self {
            Self::AuthError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_auth_error(self) -> Option<SubtitlingAuthError> {
        match self {
            Self::AuthError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited { .. })
    }

    pub fn as_rate_limited(&self) -> Option<&SubtitlingRateLimitedError> {
        match self {
            Self::RateLimited { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_rate_limited(self) -> Option<SubtitlingRateLimitedError> {
        match self {
            Self::RateLimited { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_queue_overflow(&self) -> bool {
        matches!(self, Self::QueueOverflow { .. })
    }

    pub fn as_queue_overflow(&self) -> Option<&SubtitlingQueueOverflowError> {
        match self {
            Self::QueueOverflow { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_queue_overflow(self) -> Option<SubtitlingQueueOverflowError> {
        match self {
            Self::QueueOverflow { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_input_error(&self) -> bool {
        matches!(self, Self::InputError { .. })
    }

    pub fn as_input_error(&self) -> Option<&SubtitlingInputError> {
        match self {
            Self::InputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_input_error(self) -> Option<SubtitlingInputError> {
        match self {
            Self::InputError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_transcriber_error(&self) -> bool {
        matches!(self, Self::TranscriberError { .. })
    }

    pub fn as_transcriber_error(&self) -> Option<&SubtitlingTranscriberError> {
        match self {
            Self::TranscriberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_transcriber_error(self) -> Option<SubtitlingTranscriberError> {
        match self {
            Self::TranscriberError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_translation_error(&self) -> bool {
        matches!(self, Self::TranslationError { .. })
    }

    pub fn as_translation_error(&self) -> Option<&SubtitlingTranslationError> {
        match self {
            Self::TranslationError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_translation_error(self) -> Option<SubtitlingTranslationError> {
        match self {
            Self::TranslationError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_quota_exceeded(&self) -> bool {
        matches!(self, Self::QuotaExceeded { .. })
    }

    pub fn as_quota_exceeded(&self) -> Option<&SubtitlingQuotaExceededError> {
        match self {
            Self::QuotaExceeded { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_quota_exceeded(self) -> Option<SubtitlingQuotaExceededError> {
        match self {
            Self::QuotaExceeded { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_payment_issue(&self) -> bool {
        matches!(self, Self::PaymentIssue { .. })
    }

    pub fn as_payment_issue(&self) -> Option<&SubtitlingPaymentIssueError> {
        match self {
            Self::PaymentIssue { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_payment_issue(self) -> Option<SubtitlingPaymentIssueError> {
        match self {
            Self::PaymentIssue { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_unaccepted_terms(&self) -> bool {
        matches!(self, Self::UnacceptedTerms { .. })
    }

    pub fn as_unaccepted_terms(&self) -> Option<&SubtitlingUnacceptedTermsError> {
        match self {
            Self::UnacceptedTerms { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_unaccepted_terms(self) -> Option<SubtitlingUnacceptedTermsError> {
        match self {
            Self::UnacceptedTerms { data } => Some(data),
            _ => None,
        }
    }
}
