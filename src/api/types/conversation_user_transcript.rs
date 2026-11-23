pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationUserTranscript {
    pub user_transcription_event: UserTranscriptionEvent,
}

impl ConversationUserTranscript {
    pub fn builder() -> ConversationUserTranscriptBuilder {
        <ConversationUserTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationUserTranscriptBuilder {
    user_transcription_event: Option<UserTranscriptionEvent>,
}

impl ConversationUserTranscriptBuilder {
    pub fn user_transcription_event(mut self, value: UserTranscriptionEvent) -> Self {
        self.user_transcription_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationUserTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_transcription_event`](ConversationUserTranscriptBuilder::user_transcription_event)
    pub fn build(self) -> Result<ConversationUserTranscript, BuildError> {
        Ok(ConversationUserTranscript {
            user_transcription_event: self
                .user_transcription_event
                .ok_or_else(|| BuildError::missing_field("user_transcription_event"))?,
        })
    }
}
