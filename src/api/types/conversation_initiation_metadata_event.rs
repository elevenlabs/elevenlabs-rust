pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationInitiationMetadataEvent {
    #[serde(default)]
    pub conversation_id: String,
    /// Audio format specification for agent's speech output.
    pub agent_output_audio_format: AgentOutputAudioFormat,
    /// Audio format specification for user's speech input.
    pub user_input_audio_format: UserInputAudioFormat,
}

impl ConversationInitiationMetadataEvent {
    pub fn builder() -> ConversationInitiationMetadataEventBuilder {
        <ConversationInitiationMetadataEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationMetadataEventBuilder {
    conversation_id: Option<String>,
    agent_output_audio_format: Option<AgentOutputAudioFormat>,
    user_input_audio_format: Option<UserInputAudioFormat>,
}

impl ConversationInitiationMetadataEventBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn agent_output_audio_format(mut self, value: AgentOutputAudioFormat) -> Self {
        self.agent_output_audio_format = Some(value);
        self
    }

    pub fn user_input_audio_format(mut self, value: UserInputAudioFormat) -> Self {
        self.user_input_audio_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationMetadataEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](ConversationInitiationMetadataEventBuilder::conversation_id)
    /// - [`agent_output_audio_format`](ConversationInitiationMetadataEventBuilder::agent_output_audio_format)
    /// - [`user_input_audio_format`](ConversationInitiationMetadataEventBuilder::user_input_audio_format)
    pub fn build(self) -> Result<ConversationInitiationMetadataEvent, BuildError> {
        Ok(ConversationInitiationMetadataEvent {
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            agent_output_audio_format: self
                .agent_output_audio_format
                .ok_or_else(|| BuildError::missing_field("agent_output_audio_format"))?,
            user_input_audio_format: self
                .user_input_audio_format
                .ok_or_else(|| BuildError::missing_field("user_input_audio_format"))?,
        })
    }
}
