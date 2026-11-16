pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ZendeskPlatformMetadata {
    #[serde(default)]
    pub ticket_id: String,
}

impl ZendeskPlatformMetadata {
    pub fn builder() -> ZendeskPlatformMetadataBuilder {
        <ZendeskPlatformMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZendeskPlatformMetadataBuilder {
    ticket_id: Option<String>,
}

impl ZendeskPlatformMetadataBuilder {
    pub fn ticket_id(mut self, value: impl Into<String>) -> Self {
        self.ticket_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ZendeskPlatformMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ticket_id`](ZendeskPlatformMetadataBuilder::ticket_id)
    pub fn build(self) -> Result<ZendeskPlatformMetadata, BuildError> {
        Ok(ZendeskPlatformMetadata {
            ticket_id: self
                .ticket_id
                .ok_or_else(|| BuildError::missing_field("ticket_id"))?,
        })
    }
}
