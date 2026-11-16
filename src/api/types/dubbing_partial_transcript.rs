pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingPartialTranscript {
    #[serde(default)]
    pub text: String,
}

impl DubbingPartialTranscript {
    pub fn builder() -> DubbingPartialTranscriptBuilder {
        <DubbingPartialTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingPartialTranscriptBuilder {
    text: Option<String>,
}

impl DubbingPartialTranscriptBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingPartialTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](DubbingPartialTranscriptBuilder::text)
    pub fn build(self) -> Result<DubbingPartialTranscript, BuildError> {
        Ok(DubbingPartialTranscript {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
