pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlainPlatformMetadata {
    #[serde(default)]
    pub thread_id: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl PlainPlatformMetadata {
    pub fn builder() -> PlainPlatformMetadataBuilder {
        <PlainPlatformMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlainPlatformMetadataBuilder {
    thread_id: Option<String>,
    workspace_id: Option<String>,
}

impl PlainPlatformMetadataBuilder {
    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlainPlatformMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_id`](PlainPlatformMetadataBuilder::thread_id)
    /// - [`workspace_id`](PlainPlatformMetadataBuilder::workspace_id)
    pub fn build(self) -> Result<PlainPlatformMetadata, BuildError> {
        Ok(PlainPlatformMetadata {
            thread_id: self
                .thread_id
                .ok_or_else(|| BuildError::missing_field("thread_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
