pub use crate::prelude::*;

/// Media passed inline as base64. Stored as an ephemeral asset with no
/// guaranteed retention. To keep an input and reuse it, upload via
/// ``POST /v1/assets`` and pass an ``asset`` reference instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InlineBase64Reference {
    /// The media file's bytes, base64-encoded (standard alphabet). Up to 25MB decoded.
    #[serde(rename = "content_base64")]
    #[serde(default)]
    pub content_base_64: String,
    /// The MIME type of the encoded media.
    pub mime_type: InlineBase64ReferenceMimeType,
}

impl InlineBase64Reference {
    pub fn builder() -> InlineBase64ReferenceBuilder {
        <InlineBase64ReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InlineBase64ReferenceBuilder {
    content_base_64: Option<String>,
    mime_type: Option<InlineBase64ReferenceMimeType>,
}

impl InlineBase64ReferenceBuilder {
    pub fn content_base_64(mut self, value: impl Into<String>) -> Self {
        self.content_base_64 = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: InlineBase64ReferenceMimeType) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InlineBase64Reference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_base_64`](InlineBase64ReferenceBuilder::content_base_64)
    /// - [`mime_type`](InlineBase64ReferenceBuilder::mime_type)
    pub fn build(self) -> Result<InlineBase64Reference, BuildError> {
        Ok(InlineBase64Reference {
            content_base_64: self
                .content_base_64
                .ok_or_else(|| BuildError::missing_field("content_base_64"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
        })
    }
}
