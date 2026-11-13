pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuardrailClassifierVerdict {
    /// Classifier score for the triggered category, from 0 to 1.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub score: f64,
    /// Score at or above which the guardrail triggers.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub threshold: f64,
}

impl GuardrailClassifierVerdict {
    pub fn builder() -> GuardrailClassifierVerdictBuilder {
        <GuardrailClassifierVerdictBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailClassifierVerdictBuilder {
    score: Option<f64>,
    threshold: Option<f64>,
}

impl GuardrailClassifierVerdictBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GuardrailClassifierVerdict`].
    /// This method will fail if any of the following fields are not set:
    /// - [`score`](GuardrailClassifierVerdictBuilder::score)
    /// - [`threshold`](GuardrailClassifierVerdictBuilder::threshold)
    pub fn build(self) -> Result<GuardrailClassifierVerdict, BuildError> {
        Ok(GuardrailClassifierVerdict {
            score: self
                .score
                .ok_or_else(|| BuildError::missing_field("score"))?,
            threshold: self
                .threshold
                .ok_or_else(|| BuildError::missing_field("threshold"))?,
        })
    }
}
