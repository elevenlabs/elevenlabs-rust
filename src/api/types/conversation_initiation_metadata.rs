pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationInitiationMetadata {
    pub conversation_initiation_metadata_event: ConversationInitiationMetadataEvent,
}

impl ConversationInitiationMetadata {
    pub fn builder() -> ConversationInitiationMetadataBuilder {
        <ConversationInitiationMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationMetadataBuilder {
    conversation_initiation_metadata_event: Option<ConversationInitiationMetadataEvent>,
}

impl ConversationInitiationMetadataBuilder {
    pub fn conversation_initiation_metadata_event(
        mut self,
        value: ConversationInitiationMetadataEvent,
    ) -> Self {
        self.conversation_initiation_metadata_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_initiation_metadata_event`](ConversationInitiationMetadataBuilder::conversation_initiation_metadata_event)
    pub fn build(self) -> Result<ConversationInitiationMetadata, BuildError> {
        Ok(ConversationInitiationMetadata {
            conversation_initiation_metadata_event: self
                .conversation_initiation_metadata_event
                .ok_or_else(|| {
                    BuildError::missing_field("conversation_initiation_metadata_event")
                })?,
        })
    }
}
