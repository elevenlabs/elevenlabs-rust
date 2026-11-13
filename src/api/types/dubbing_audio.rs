pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingAudio {
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub sample_rate: i64,
}

impl DubbingAudio {
    pub fn builder() -> DubbingAudioBuilder {
        <DubbingAudioBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingAudioBuilder {
    data: Option<String>,
    sample_rate: Option<i64>,
}

impl DubbingAudioBuilder {
    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingAudio`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](DubbingAudioBuilder::data)
    /// - [`sample_rate`](DubbingAudioBuilder::sample_rate)
    pub fn build(self) -> Result<DubbingAudio, BuildError> {
        Ok(DubbingAudio {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            sample_rate: self
                .sample_rate
                .ok_or_else(|| BuildError::missing_field("sample_rate"))?,
        })
    }
}
