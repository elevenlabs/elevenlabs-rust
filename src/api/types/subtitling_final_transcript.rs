pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingFinalTranscript {
    #[serde(default)]
    pub text: String,
}

impl SubtitlingFinalTranscript {
    pub fn builder() -> SubtitlingFinalTranscriptBuilder {
        <SubtitlingFinalTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingFinalTranscriptBuilder {
    text: Option<String>,
}

impl SubtitlingFinalTranscriptBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingFinalTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SubtitlingFinalTranscriptBuilder::text)
    pub fn build(self) -> Result<SubtitlingFinalTranscript, BuildError> {
        Ok(SubtitlingFinalTranscript {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
