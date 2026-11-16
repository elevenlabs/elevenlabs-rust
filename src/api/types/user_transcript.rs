pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserTranscript {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(default)]
    pub user_transcript: Vec<UserTranscriptUserTranscriptItem>,
}

impl UserTranscript {
    pub fn builder() -> UserTranscriptBuilder {
        <UserTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserTranscriptBuilder {
    event_id: Option<i64>,
    user_transcript: Option<Vec<UserTranscriptUserTranscriptItem>>,
}

impl UserTranscriptBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn user_transcript(mut self, value: Vec<UserTranscriptUserTranscriptItem>) -> Self {
        self.user_transcript = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](UserTranscriptBuilder::event_id)
    /// - [`user_transcript`](UserTranscriptBuilder::user_transcript)
    pub fn build(self) -> Result<UserTranscript, BuildError> {
        Ok(UserTranscript {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            user_transcript: self
                .user_transcript
                .ok_or_else(|| BuildError::missing_field("user_transcript"))?,
        })
    }
}
