pub use crate::prelude::*;

/// A completed media generation and its output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MediaGenerationCompletedResponse {
    /// The unique identifier of the generation.
    #[serde(default)]
    pub id: String,
    /// A signed URL to download the generated media from. It expires about an hour after this response is returned; fetch the generation again for a fresh URL.
    #[serde(default)]
    pub content_url: String,
    /// The MIME type of the generated media.
    #[serde(default)]
    pub content_mime_type: String,
}

impl MediaGenerationCompletedResponse {
    pub fn builder() -> MediaGenerationCompletedResponseBuilder {
        <MediaGenerationCompletedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaGenerationCompletedResponseBuilder {
    id: Option<String>,
    content_url: Option<String>,
    content_mime_type: Option<String>,
}

impl MediaGenerationCompletedResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content_url(mut self, value: impl Into<String>) -> Self {
        self.content_url = Some(value.into());
        self
    }

    pub fn content_mime_type(mut self, value: impl Into<String>) -> Self {
        self.content_mime_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaGenerationCompletedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MediaGenerationCompletedResponseBuilder::id)
    /// - [`content_url`](MediaGenerationCompletedResponseBuilder::content_url)
    /// - [`content_mime_type`](MediaGenerationCompletedResponseBuilder::content_mime_type)
    pub fn build(self) -> Result<MediaGenerationCompletedResponse, BuildError> {
        Ok(MediaGenerationCompletedResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content_url: self
                .content_url
                .ok_or_else(|| BuildError::missing_field("content_url"))?,
            content_mime_type: self
                .content_mime_type
                .ok_or_else(|| BuildError::missing_field("content_mime_type"))?,
        })
    }
}
