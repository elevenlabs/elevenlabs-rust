pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NumericUniformScoring {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub max_score: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

impl NumericUniformScoring {
    pub fn builder() -> NumericUniformScoringBuilder {
        <NumericUniformScoringBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NumericUniformScoringBuilder {
    max_score: Option<i64>,
    instructions: Option<String>,
}

impl NumericUniformScoringBuilder {
    pub fn max_score(mut self, value: i64) -> Self {
        self.max_score = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NumericUniformScoring`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_score`](NumericUniformScoringBuilder::max_score)
    pub fn build(self) -> Result<NumericUniformScoring, BuildError> {
        Ok(NumericUniformScoring {
            max_score: self
                .max_score
                .ok_or_else(|| BuildError::missing_field("max_score"))?,
            instructions: self.instructions,
        })
    }
}
