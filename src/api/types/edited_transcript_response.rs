pub use crate::prelude::*;

/// A transcript produced by the optional transcript edit (text only, untimed).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EditedTranscriptResponse {
    /// The edited transcript text.
    #[serde(default)]
    pub text: String,
    /// The edited transcript text. Same value as `text`; matches the `edited_text` field of the realtime `edited_transcript` event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_text: Option<String>,
}

impl EditedTranscriptResponse {
    pub fn builder() -> EditedTranscriptResponseBuilder {
        <EditedTranscriptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditedTranscriptResponseBuilder {
    text: Option<String>,
    edited_text: Option<String>,
}

impl EditedTranscriptResponseBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn edited_text(mut self, value: impl Into<String>) -> Self {
        self.edited_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditedTranscriptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](EditedTranscriptResponseBuilder::text)
    pub fn build(self) -> Result<EditedTranscriptResponse, BuildError> {
        Ok(EditedTranscriptResponse {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            edited_text: self.edited_text,
        })
    }
}
