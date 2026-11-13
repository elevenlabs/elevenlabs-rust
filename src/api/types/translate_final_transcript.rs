pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateFinalTranscript {
    #[serde(default)]
    pub text: String,
}

impl TranslateFinalTranscript {
    pub fn builder() -> TranslateFinalTranscriptBuilder {
        <TranslateFinalTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateFinalTranscriptBuilder {
    text: Option<String>,
}

impl TranslateFinalTranscriptBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateFinalTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TranslateFinalTranscriptBuilder::text)
    pub fn build(self) -> Result<TranslateFinalTranscript, BuildError> {
        Ok(TranslateFinalTranscript {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
