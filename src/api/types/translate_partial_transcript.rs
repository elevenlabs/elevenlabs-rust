pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslatePartialTranscript {
    #[serde(default)]
    pub text: String,
}

impl TranslatePartialTranscript {
    pub fn builder() -> TranslatePartialTranscriptBuilder {
        <TranslatePartialTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslatePartialTranscriptBuilder {
    text: Option<String>,
}

impl TranslatePartialTranscriptBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslatePartialTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TranslatePartialTranscriptBuilder::text)
    pub fn build(self) -> Result<TranslatePartialTranscript, BuildError> {
        Ok(TranslatePartialTranscript {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
