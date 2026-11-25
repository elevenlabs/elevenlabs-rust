pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingPartialTranscript {
    #[serde(default)]
    pub text: String,
}

impl SubtitlingPartialTranscript {
    pub fn builder() -> SubtitlingPartialTranscriptBuilder {
        <SubtitlingPartialTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingPartialTranscriptBuilder {
    text: Option<String>,
}

impl SubtitlingPartialTranscriptBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingPartialTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SubtitlingPartialTranscriptBuilder::text)
    pub fn build(self) -> Result<SubtitlingPartialTranscript, BuildError> {
        Ok(SubtitlingPartialTranscript {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
