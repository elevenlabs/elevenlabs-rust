pub use crate::prelude::*;

/// An `audio` output, downloaded from `content_url` once completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TemplateAudioOutput {
    /// The id of the generation behind this output. Pass it as a `generation` reference to use the output as an input elsewhere.
    #[serde(default)]
    pub id: String,
    /// Where a run, or one of its outputs, is in its lifecycle. The run's is rolled
    /// up from its outputs' (``aggregate_output_statuses``). A class rather than a
    /// ``Literal`` so it publishes as one named enum instead of being inlined into
    /// every kind.
    pub status: TemplateRunStatus,
    /// The category of failure. Present only when `status` is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TemplateArrayOutputFailureReason>,
    /// A human-readable description of the failure. Present only when `status` is `failed`. Failed generations are not charged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// A signed URL to download the generated media from. Present only when `status` is `completed`. It expires about an hour after this response is returned; fetch the run again for a fresh URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    /// The MIME type of the generated media. Present only when `status` is `completed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_mime_type: Option<String>,
}

impl TemplateAudioOutput {
    pub fn builder() -> TemplateAudioOutputBuilder {
        <TemplateAudioOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateAudioOutputBuilder {
    id: Option<String>,
    status: Option<TemplateRunStatus>,
    failure_reason: Option<TemplateArrayOutputFailureReason>,
    error_message: Option<String>,
    content_url: Option<String>,
    content_mime_type: Option<String>,
}

impl TemplateAudioOutputBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: TemplateRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn failure_reason(mut self, value: TemplateArrayOutputFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
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

    /// Consumes the builder and constructs a [`TemplateAudioOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplateAudioOutputBuilder::id)
    /// - [`status`](TemplateAudioOutputBuilder::status)
    pub fn build(self) -> Result<TemplateAudioOutput, BuildError> {
        Ok(TemplateAudioOutput {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            failure_reason: self.failure_reason,
            error_message: self.error_message,
            content_url: self.content_url,
            content_mime_type: self.content_mime_type,
        })
    }
}
