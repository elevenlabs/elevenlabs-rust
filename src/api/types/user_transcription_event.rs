pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserTranscriptionEvent {
    #[serde(default)]
    pub user_transcript: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
}

impl UserTranscriptionEvent {
    pub fn builder() -> UserTranscriptionEventBuilder {
        <UserTranscriptionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserTranscriptionEventBuilder {
    user_transcript: Option<String>,
    event_id: Option<i64>,
}

impl UserTranscriptionEventBuilder {
    pub fn user_transcript(mut self, value: impl Into<String>) -> Self {
        self.user_transcript = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserTranscriptionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_transcript`](UserTranscriptionEventBuilder::user_transcript)
    /// - [`event_id`](UserTranscriptionEventBuilder::event_id)
    pub fn build(self) -> Result<UserTranscriptionEvent, BuildError> {
        Ok(UserTranscriptionEvent {
            user_transcript: self
                .user_transcript
                .ok_or_else(|| BuildError::missing_field("user_transcript"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}
