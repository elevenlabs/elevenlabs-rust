pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartialTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<DubbingPartialTranscriptMessageType>,
    pub text: String,
    /// Probability that the speaker has finished their turn. Only sent for models that predict turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub turn_probability: Option<f64>,
    /// Draft words with speaker IDs when diarize is enabled. Speaker IDs and timestamps may change as more audio is received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<PartialTranscriptWordsItem>>,
}

impl PartialTranscript {
    pub fn builder() -> PartialTranscriptBuilder {
        <PartialTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartialTranscriptBuilder {
    message_type: Option<DubbingPartialTranscriptMessageType>,
    text: Option<String>,
    turn_probability: Option<f64>,
    words: Option<Vec<PartialTranscriptWordsItem>>,
}

impl PartialTranscriptBuilder {
    pub fn message_type(mut self, value: DubbingPartialTranscriptMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn turn_probability(mut self, value: f64) -> Self {
        self.turn_probability = Some(value);
        self
    }

    pub fn words(mut self, value: Vec<PartialTranscriptWordsItem>) -> Self {
        self.words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartialTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PartialTranscriptBuilder::text)
    pub fn build(self) -> Result<PartialTranscript, BuildError> {
        Ok(PartialTranscript {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            turn_probability: self.turn_probability,
            words: self.words,
        })
    }
}
