pub use crate::prelude::*;

/// Audio passed inline as base64. The audio is stored as an ephemeral
/// asset with no guaranteed retention: it may be deleted at any time after
/// the generation completes. To keep an input and reuse it across
/// generations, upload it via the assets API (`POST /v1/assets`) and pass
/// an `asset` reference instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InlineAudioReference {
    /// The media file's bytes, base64-encoded (standard alphabet). Up to 25MB decoded.
    #[serde(rename = "content_base64")]
    #[serde(default)]
    pub content_base_64: String,
    /// The MIME type of the encoded audio.
    pub mime_type: InlineAudioReferenceMimeType,
}

impl InlineAudioReference {
    pub fn builder() -> InlineAudioReferenceBuilder {
        <InlineAudioReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InlineAudioReferenceBuilder {
    content_base_64: Option<String>,
    mime_type: Option<InlineAudioReferenceMimeType>,
}

impl InlineAudioReferenceBuilder {
    pub fn content_base_64(mut self, value: impl Into<String>) -> Self {
        self.content_base_64 = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: InlineAudioReferenceMimeType) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InlineAudioReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_base_64`](InlineAudioReferenceBuilder::content_base_64)
    /// - [`mime_type`](InlineAudioReferenceBuilder::mime_type)
    pub fn build(self) -> Result<InlineAudioReference, BuildError> {
        Ok(InlineAudioReference {
            content_base_64: self
                .content_base_64
                .ok_or_else(|| BuildError::missing_field("content_base_64"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
        })
    }
}
