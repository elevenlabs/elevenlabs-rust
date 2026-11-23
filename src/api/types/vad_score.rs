pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VadScore {
    pub vad_score_event: VadScoreEvent,
}

impl VadScore {
    pub fn builder() -> VadScoreBuilder {
        <VadScoreBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VadScoreBuilder {
    vad_score_event: Option<VadScoreEvent>,
}

impl VadScoreBuilder {
    pub fn vad_score_event(mut self, value: VadScoreEvent) -> Self {
        self.vad_score_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VadScore`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vad_score_event`](VadScoreBuilder::vad_score_event)
    pub fn build(self) -> Result<VadScore, BuildError> {
        Ok(VadScore {
            vad_score_event: self
                .vad_score_event
                .ok_or_else(|| BuildError::missing_field("vad_score_event"))?,
        })
    }
}
