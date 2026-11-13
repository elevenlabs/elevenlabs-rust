pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateAudio {
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub sample_rate: i64,
}

impl TranslateAudio {
    pub fn builder() -> TranslateAudioBuilder {
        <TranslateAudioBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateAudioBuilder {
    data: Option<String>,
    sample_rate: Option<i64>,
}

impl TranslateAudioBuilder {
    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranslateAudio`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](TranslateAudioBuilder::data)
    /// - [`sample_rate`](TranslateAudioBuilder::sample_rate)
    pub fn build(self) -> Result<TranslateAudio, BuildError> {
        Ok(TranslateAudio {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            sample_rate: self
                .sample_rate
                .ok_or_else(|| BuildError::missing_field("sample_rate"))?,
        })
    }
}
