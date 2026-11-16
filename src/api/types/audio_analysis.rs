pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioAnalysis {
    pub status: AssetTranscriptionStatus,
    pub data: Option<AudioAnalysisResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub updated_at_ms: Option<i64>,
}

impl AudioAnalysis {
    pub fn builder() -> AudioAnalysisBuilder {
        <AudioAnalysisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAnalysisBuilder {
    status: Option<AssetTranscriptionStatus>,
    data: Option<AudioAnalysisResult>,
    updated_at_ms: Option<i64>,
}

impl AudioAnalysisBuilder {
    pub fn status(mut self, value: AssetTranscriptionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn data(mut self, value: AudioAnalysisResult) -> Self {
        self.data = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioAnalysis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AudioAnalysisBuilder::status)
    pub fn build(self) -> Result<AudioAnalysis, BuildError> {
        Ok(AudioAnalysis {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            data: self.data,
            updated_at_ms: self.updated_at_ms,
        })
    }
}
