pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommittedTranscriptWithTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<CommittedTranscriptWithTimestampsMessageType>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<CommittedTranscriptWithTimestampsWordsItem>>,
}

impl CommittedTranscriptWithTimestamps {
    pub fn builder() -> CommittedTranscriptWithTimestampsBuilder {
        <CommittedTranscriptWithTimestampsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommittedTranscriptWithTimestampsBuilder {
    message_type: Option<CommittedTranscriptWithTimestampsMessageType>,
    text: Option<String>,
    language_code: Option<String>,
    words: Option<Vec<CommittedTranscriptWithTimestampsWordsItem>>,
}

impl CommittedTranscriptWithTimestampsBuilder {
    pub fn message_type(mut self, value: CommittedTranscriptWithTimestampsMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn words(mut self, value: Vec<CommittedTranscriptWithTimestampsWordsItem>) -> Self {
        self.words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommittedTranscriptWithTimestamps`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CommittedTranscriptWithTimestampsBuilder::text)
    pub fn build(self) -> Result<CommittedTranscriptWithTimestamps, BuildError> {
        Ok(CommittedTranscriptWithTimestamps {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            language_code: self.language_code,
            words: self.words,
        })
    }
}
