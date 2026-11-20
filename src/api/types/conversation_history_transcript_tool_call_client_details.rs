pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationHistoryTranscriptToolCallClientDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConversationHistoryTranscriptToolCallClientDetailsType>,
    pub parameters: String,
}

impl ConversationHistoryTranscriptToolCallClientDetails {
    pub fn builder() -> ConversationHistoryTranscriptToolCallClientDetailsBuilder {
        <ConversationHistoryTranscriptToolCallClientDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallClientDetailsBuilder {
    r#type: Option<ConversationHistoryTranscriptToolCallClientDetailsType>,
    parameters: Option<String>,
}

impl ConversationHistoryTranscriptToolCallClientDetailsBuilder {
    pub fn r#type(mut self, value: ConversationHistoryTranscriptToolCallClientDetailsType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn parameters(mut self, value: impl Into<String>) -> Self {
        self.parameters = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallClientDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parameters`](ConversationHistoryTranscriptToolCallClientDetailsBuilder::parameters)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallClientDetails, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallClientDetails {
            r#type: self.r#type,
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
