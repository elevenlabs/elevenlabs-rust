pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartialTranscriptWordsCharactersItem {
    /// The character that was transcribed.
    #[serde(default)]
    pub text: String,
    /// The start time of the character in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub start: Option<f64>,
    /// The end time of the character in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub end: Option<f64>,
}

impl PartialTranscriptWordsCharactersItem {
    pub fn builder() -> PartialTranscriptWordsCharactersItemBuilder {
        <PartialTranscriptWordsCharactersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartialTranscriptWordsCharactersItemBuilder {
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
}

impl PartialTranscriptWordsCharactersItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartialTranscriptWordsCharactersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PartialTranscriptWordsCharactersItemBuilder::text)
    pub fn build(self) -> Result<PartialTranscriptWordsCharactersItem, BuildError> {
        Ok(PartialTranscriptWordsCharactersItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            start: self.start,
            end: self.end,
        })
    }
}
