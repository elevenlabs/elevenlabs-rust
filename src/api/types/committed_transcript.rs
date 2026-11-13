pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommittedTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<CommittedTranscriptMessageType>,
    pub text: String,
    /// Zero-based commit index when diarize is enabled. Matches the segment_index in the corresponding diarized_transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub segment_index: Option<i64>,
}

impl CommittedTranscript {
    pub fn builder() -> CommittedTranscriptBuilder {
        <CommittedTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommittedTranscriptBuilder {
    message_type: Option<CommittedTranscriptMessageType>,
    text: Option<String>,
    segment_index: Option<i64>,
}

impl CommittedTranscriptBuilder {
    pub fn message_type(mut self, value: CommittedTranscriptMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn segment_index(mut self, value: i64) -> Self {
        self.segment_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommittedTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CommittedTranscriptBuilder::text)
    pub fn build(self) -> Result<CommittedTranscript, BuildError> {
        Ok(CommittedTranscript {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            segment_index: self.segment_index,
        })
    }
}
