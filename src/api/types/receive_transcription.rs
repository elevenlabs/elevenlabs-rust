pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveTranscription {
    SessionStarted(SessionStarted),
    PartialTranscript(PartialTranscript),
    CommittedTranscript(CommittedTranscript),
    DiarizedTranscript(DiarizedTranscript),
    SpeakersMerged(SpeakersMerged),
    CommittedTranscriptWithTimestamps(CommittedTranscriptWithTimestamps),
    CommittedTranscriptEntities(CommittedTranscriptEntities),
    RedactedTranscript(RedactedTranscript),
    EditedTranscript(EditedTranscript),
    ScribeWarning(ScribeWarning),
    ScribeError(ScribeError),
    ScribeAuthError(ScribeAuthError),
    ScribeQuotaExceededError(ScribeQuotaExceededError),
    ScribeThrottledError(ScribeThrottledError),
    ScribeUnacceptedTermsError(ScribeUnacceptedTermsError),
    ScribeRateLimitedError(ScribeRateLimitedError),
    ScribeQueueOverflowError(ScribeQueueOverflowError),
    ScribeResourceExhaustedError(ScribeResourceExhaustedError),
    ScribeSessionTimeLimitExceededError(ScribeSessionTimeLimitExceededError),
    ScribeInputError(ScribeInputError),
    ScribeInvalidRequestError(ScribeInvalidRequestError),
    ScribeChunkSizeExceededError(ScribeChunkSizeExceededError),
    ScribeInsufficientAudioActivityError(ScribeInsufficientAudioActivityError),
    ScribeTranscriberError(ScribeTranscriberError),
}

impl<'de> serde::Deserialize<'de> for ReceiveTranscription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("session_started")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<SessionStarted>(value.clone()) {
                return Ok(Self::SessionStarted(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("partial_transcript")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<PartialTranscript>(value.clone()) {
                return Ok(Self::PartialTranscript(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("committed_transcript")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<CommittedTranscript>(value.clone()) {
                return Ok(Self::CommittedTranscript(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("diarized_transcript")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<DiarizedTranscript>(value.clone()) {
                return Ok(Self::DiarizedTranscript(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("speakers_merged")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<SpeakersMerged>(value.clone()) {
                return Ok(Self::SpeakersMerged(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("committed_transcript_with_timestamps")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<CommittedTranscriptWithTimestamps>(value.clone())
            {
                return Ok(Self::CommittedTranscriptWithTimestamps(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("committed_transcript_entities")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<CommittedTranscriptEntities>(value.clone())
            {
                return Ok(Self::CommittedTranscriptEntities(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("redacted_transcript")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<RedactedTranscript>(value.clone()) {
                return Ok(Self::RedactedTranscript(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("warning")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeWarning>(value.clone()) {
                return Ok(Self::ScribeWarning(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("error")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeError>(value.clone()) {
                return Ok(Self::ScribeError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("auth_error")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeAuthError>(value.clone()) {
                return Ok(Self::ScribeAuthError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("quota_exceeded")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeQuotaExceededError>(value.clone()) {
                return Ok(Self::ScribeQuotaExceededError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("commit_throttled")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeThrottledError>(value.clone()) {
                return Ok(Self::ScribeThrottledError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("unaccepted_terms")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeUnacceptedTermsError>(value.clone())
            {
                return Ok(Self::ScribeUnacceptedTermsError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("rate_limited")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeRateLimitedError>(value.clone()) {
                return Ok(Self::ScribeRateLimitedError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("queue_overflow")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeQueueOverflowError>(value.clone()) {
                return Ok(Self::ScribeQueueOverflowError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("resource_exhausted")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ScribeResourceExhaustedError>(value.clone())
            {
                return Ok(Self::ScribeResourceExhaustedError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("session_time_limit_exceeded")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ScribeSessionTimeLimitExceededError>(value.clone())
            {
                return Ok(Self::ScribeSessionTimeLimitExceededError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("input_error")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeInputError>(value.clone()) {
                return Ok(Self::ScribeInputError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("invalid_request")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeInvalidRequestError>(value.clone()) {
                return Ok(Self::ScribeInvalidRequestError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("chunk_size_exceeded")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ScribeChunkSizeExceededError>(value.clone())
            {
                return Ok(Self::ScribeChunkSizeExceededError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("insufficient_audio_activity")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ScribeInsufficientAudioActivityError>(value.clone())
            {
                return Ok(Self::ScribeInsufficientAudioActivityError(parsed));
            }
        }

        if value
            .get("message_type")
            .and_then(serde_json::Value::as_str)
            == Some("transcriber_error")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ScribeTranscriberError>(value.clone()) {
                return Ok(Self::ScribeTranscriberError(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<SessionStarted>(value.clone()) {
                return Ok(Self::SessionStarted(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<PartialTranscript>(value.clone()) {
                return Ok(Self::PartialTranscript(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<CommittedTranscript>(value.clone()) {
                return Ok(Self::CommittedTranscript(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<DiarizedTranscript>(value.clone()) {
                return Ok(Self::DiarizedTranscript(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SpeakersMerged>(value.clone()) {
                return Ok(Self::SpeakersMerged(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<CommittedTranscriptWithTimestamps>(value.clone())
            {
                return Ok(Self::CommittedTranscriptWithTimestamps(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<CommittedTranscriptEntities>(value.clone())
            {
                return Ok(Self::CommittedTranscriptEntities(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<RedactedTranscript>(value.clone()) {
                return Ok(Self::RedactedTranscript(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<EditedTranscript>(value.clone()) {
                return Ok(Self::EditedTranscript(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeWarning>(value.clone()) {
                return Ok(Self::ScribeWarning(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeError>(value.clone()) {
                return Ok(Self::ScribeError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeAuthError>(value.clone()) {
                return Ok(Self::ScribeAuthError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeQuotaExceededError>(value.clone()) {
                return Ok(Self::ScribeQuotaExceededError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeThrottledError>(value.clone()) {
                return Ok(Self::ScribeThrottledError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeUnacceptedTermsError>(value.clone())
            {
                return Ok(Self::ScribeUnacceptedTermsError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeRateLimitedError>(value.clone()) {
                return Ok(Self::ScribeRateLimitedError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeQueueOverflowError>(value.clone()) {
                return Ok(Self::ScribeQueueOverflowError(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<ScribeResourceExhaustedError>(value.clone())
            {
                return Ok(Self::ScribeResourceExhaustedError(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<ScribeSessionTimeLimitExceededError>(value.clone())
            {
                return Ok(Self::ScribeSessionTimeLimitExceededError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeInputError>(value.clone()) {
                return Ok(Self::ScribeInputError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeInvalidRequestError>(value.clone()) {
                return Ok(Self::ScribeInvalidRequestError(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<ScribeChunkSizeExceededError>(value.clone())
            {
                return Ok(Self::ScribeChunkSizeExceededError(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<ScribeInsufficientAudioActivityError>(value.clone())
            {
                return Ok(Self::ScribeInsufficientAudioActivityError(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ScribeTranscriberError>(value.clone()) {
                return Ok(Self::ScribeTranscriberError(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ReceiveTranscription",
        ))
    }
}

impl ReceiveTranscription {
    pub fn is_session_started(&self) -> bool {
        matches!(self, Self::SessionStarted(_))
    }

    pub fn as_session_started(&self) -> Option<&SessionStarted> {
        match self {
            Self::SessionStarted(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_session_started(self) -> Option<SessionStarted> {
        match self {
            Self::SessionStarted(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_partial_transcript(&self) -> bool {
        matches!(self, Self::PartialTranscript(_))
    }

    pub fn as_partial_transcript(&self) -> Option<&PartialTranscript> {
        match self {
            Self::PartialTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_partial_transcript(self) -> Option<PartialTranscript> {
        match self {
            Self::PartialTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_committed_transcript(&self) -> bool {
        matches!(self, Self::CommittedTranscript(_))
    }

    pub fn as_committed_transcript(&self) -> Option<&CommittedTranscript> {
        match self {
            Self::CommittedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_committed_transcript(self) -> Option<CommittedTranscript> {
        match self {
            Self::CommittedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_diarized_transcript(&self) -> bool {
        matches!(self, Self::DiarizedTranscript(_))
    }

    pub fn as_diarized_transcript(&self) -> Option<&DiarizedTranscript> {
        match self {
            Self::DiarizedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_diarized_transcript(self) -> Option<DiarizedTranscript> {
        match self {
            Self::DiarizedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_speakers_merged(&self) -> bool {
        matches!(self, Self::SpeakersMerged(_))
    }

    pub fn as_speakers_merged(&self) -> Option<&SpeakersMerged> {
        match self {
            Self::SpeakersMerged(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_speakers_merged(self) -> Option<SpeakersMerged> {
        match self {
            Self::SpeakersMerged(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_committed_transcript_with_timestamps(&self) -> bool {
        matches!(self, Self::CommittedTranscriptWithTimestamps(_))
    }

    pub fn as_committed_transcript_with_timestamps(
        &self,
    ) -> Option<&CommittedTranscriptWithTimestamps> {
        match self {
            Self::CommittedTranscriptWithTimestamps(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_committed_transcript_with_timestamps(
        self,
    ) -> Option<CommittedTranscriptWithTimestamps> {
        match self {
            Self::CommittedTranscriptWithTimestamps(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_committed_transcript_entities(&self) -> bool {
        matches!(self, Self::CommittedTranscriptEntities(_))
    }

    pub fn as_committed_transcript_entities(&self) -> Option<&CommittedTranscriptEntities> {
        match self {
            Self::CommittedTranscriptEntities(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_committed_transcript_entities(self) -> Option<CommittedTranscriptEntities> {
        match self {
            Self::CommittedTranscriptEntities(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_redacted_transcript(&self) -> bool {
        matches!(self, Self::RedactedTranscript(_))
    }

    pub fn as_redacted_transcript(&self) -> Option<&RedactedTranscript> {
        match self {
            Self::RedactedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_redacted_transcript(self) -> Option<RedactedTranscript> {
        match self {
            Self::RedactedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_edited_transcript(&self) -> bool {
        matches!(self, Self::EditedTranscript(_))
    }

    pub fn as_edited_transcript(&self) -> Option<&EditedTranscript> {
        match self {
            Self::EditedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_edited_transcript(self) -> Option<EditedTranscript> {
        match self {
            Self::EditedTranscript(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_warning(&self) -> bool {
        matches!(self, Self::ScribeWarning(_))
    }

    pub fn as_scribe_warning(&self) -> Option<&ScribeWarning> {
        match self {
            Self::ScribeWarning(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_warning(self) -> Option<ScribeWarning> {
        match self {
            Self::ScribeWarning(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_error(&self) -> bool {
        matches!(self, Self::ScribeError(_))
    }

    pub fn as_scribe_error(&self) -> Option<&ScribeError> {
        match self {
            Self::ScribeError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_error(self) -> Option<ScribeError> {
        match self {
            Self::ScribeError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_auth_error(&self) -> bool {
        matches!(self, Self::ScribeAuthError(_))
    }

    pub fn as_scribe_auth_error(&self) -> Option<&ScribeAuthError> {
        match self {
            Self::ScribeAuthError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_auth_error(self) -> Option<ScribeAuthError> {
        match self {
            Self::ScribeAuthError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_quota_exceeded_error(&self) -> bool {
        matches!(self, Self::ScribeQuotaExceededError(_))
    }

    pub fn as_scribe_quota_exceeded_error(&self) -> Option<&ScribeQuotaExceededError> {
        match self {
            Self::ScribeQuotaExceededError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_quota_exceeded_error(self) -> Option<ScribeQuotaExceededError> {
        match self {
            Self::ScribeQuotaExceededError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_throttled_error(&self) -> bool {
        matches!(self, Self::ScribeThrottledError(_))
    }

    pub fn as_scribe_throttled_error(&self) -> Option<&ScribeThrottledError> {
        match self {
            Self::ScribeThrottledError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_throttled_error(self) -> Option<ScribeThrottledError> {
        match self {
            Self::ScribeThrottledError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_unaccepted_terms_error(&self) -> bool {
        matches!(self, Self::ScribeUnacceptedTermsError(_))
    }

    pub fn as_scribe_unaccepted_terms_error(&self) -> Option<&ScribeUnacceptedTermsError> {
        match self {
            Self::ScribeUnacceptedTermsError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_unaccepted_terms_error(self) -> Option<ScribeUnacceptedTermsError> {
        match self {
            Self::ScribeUnacceptedTermsError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_rate_limited_error(&self) -> bool {
        matches!(self, Self::ScribeRateLimitedError(_))
    }

    pub fn as_scribe_rate_limited_error(&self) -> Option<&ScribeRateLimitedError> {
        match self {
            Self::ScribeRateLimitedError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_rate_limited_error(self) -> Option<ScribeRateLimitedError> {
        match self {
            Self::ScribeRateLimitedError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_queue_overflow_error(&self) -> bool {
        matches!(self, Self::ScribeQueueOverflowError(_))
    }

    pub fn as_scribe_queue_overflow_error(&self) -> Option<&ScribeQueueOverflowError> {
        match self {
            Self::ScribeQueueOverflowError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_queue_overflow_error(self) -> Option<ScribeQueueOverflowError> {
        match self {
            Self::ScribeQueueOverflowError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_resource_exhausted_error(&self) -> bool {
        matches!(self, Self::ScribeResourceExhaustedError(_))
    }

    pub fn as_scribe_resource_exhausted_error(&self) -> Option<&ScribeResourceExhaustedError> {
        match self {
            Self::ScribeResourceExhaustedError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_resource_exhausted_error(self) -> Option<ScribeResourceExhaustedError> {
        match self {
            Self::ScribeResourceExhaustedError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_session_time_limit_exceeded_error(&self) -> bool {
        matches!(self, Self::ScribeSessionTimeLimitExceededError(_))
    }

    pub fn as_scribe_session_time_limit_exceeded_error(
        &self,
    ) -> Option<&ScribeSessionTimeLimitExceededError> {
        match self {
            Self::ScribeSessionTimeLimitExceededError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_session_time_limit_exceeded_error(
        self,
    ) -> Option<ScribeSessionTimeLimitExceededError> {
        match self {
            Self::ScribeSessionTimeLimitExceededError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_input_error(&self) -> bool {
        matches!(self, Self::ScribeInputError(_))
    }

    pub fn as_scribe_input_error(&self) -> Option<&ScribeInputError> {
        match self {
            Self::ScribeInputError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_input_error(self) -> Option<ScribeInputError> {
        match self {
            Self::ScribeInputError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_invalid_request_error(&self) -> bool {
        matches!(self, Self::ScribeInvalidRequestError(_))
    }

    pub fn as_scribe_invalid_request_error(&self) -> Option<&ScribeInvalidRequestError> {
        match self {
            Self::ScribeInvalidRequestError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_invalid_request_error(self) -> Option<ScribeInvalidRequestError> {
        match self {
            Self::ScribeInvalidRequestError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_chunk_size_exceeded_error(&self) -> bool {
        matches!(self, Self::ScribeChunkSizeExceededError(_))
    }

    pub fn as_scribe_chunk_size_exceeded_error(&self) -> Option<&ScribeChunkSizeExceededError> {
        match self {
            Self::ScribeChunkSizeExceededError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_chunk_size_exceeded_error(self) -> Option<ScribeChunkSizeExceededError> {
        match self {
            Self::ScribeChunkSizeExceededError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_insufficient_audio_activity_error(&self) -> bool {
        matches!(self, Self::ScribeInsufficientAudioActivityError(_))
    }

    pub fn as_scribe_insufficient_audio_activity_error(
        &self,
    ) -> Option<&ScribeInsufficientAudioActivityError> {
        match self {
            Self::ScribeInsufficientAudioActivityError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_insufficient_audio_activity_error(
        self,
    ) -> Option<ScribeInsufficientAudioActivityError> {
        match self {
            Self::ScribeInsufficientAudioActivityError(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_scribe_transcriber_error(&self) -> bool {
        matches!(self, Self::ScribeTranscriberError(_))
    }

    pub fn as_scribe_transcriber_error(&self) -> Option<&ScribeTranscriberError> {
        match self {
            Self::ScribeTranscriberError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scribe_transcriber_error(self) -> Option<ScribeTranscriberError> {
        match self {
            Self::ScribeTranscriberError(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ReceiveTranscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionStarted(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PartialTranscript(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CommittedTranscript(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DiarizedTranscript(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SpeakersMerged(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CommittedTranscriptWithTimestamps(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CommittedTranscriptEntities(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::RedactedTranscript(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::EditedTranscript(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeWarning(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeAuthError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeQuotaExceededError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeThrottledError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeUnacceptedTermsError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeRateLimitedError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeQueueOverflowError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeResourceExhaustedError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeSessionTimeLimitExceededError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeInputError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeInvalidRequestError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeChunkSizeExceededError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeInsufficientAudioActivityError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ScribeTranscriberError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
