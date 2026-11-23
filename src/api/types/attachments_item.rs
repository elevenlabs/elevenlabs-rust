pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttachmentsItem {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl AttachmentsItem {
    pub fn builder() -> AttachmentsItemBuilder {
        <AttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachmentsItemBuilder {
    url: Option<String>,
    name: Option<String>,
    mime_type: Option<String>,
}

impl AttachmentsItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AttachmentsItemBuilder::url)
    /// - [`name`](AttachmentsItemBuilder::name)
    pub fn build(self) -> Result<AttachmentsItem, BuildError> {
        Ok(AttachmentsItem {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            mime_type: self.mime_type,
        })
    }
}
