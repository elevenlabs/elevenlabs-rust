pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VadScoreEvent {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub vad_score: f64,
}

impl VadScoreEvent {
    pub fn builder() -> VadScoreEventBuilder {
        <VadScoreEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VadScoreEventBuilder {
    vad_score: Option<f64>,
}

impl VadScoreEventBuilder {
    pub fn vad_score(mut self, value: f64) -> Self {
        self.vad_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VadScoreEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vad_score`](VadScoreEventBuilder::vad_score)
    pub fn build(self) -> Result<VadScoreEvent, BuildError> {
        Ok(VadScoreEvent {
            vad_score: self
                .vad_score
                .ok_or_else(|| BuildError::missing_field("vad_score"))?,
        })
    }
}
