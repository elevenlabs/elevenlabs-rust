pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExactParameterEvaluationStrategy {
    /// The exact string value that the parameter must match.
    #[serde(default)]
    pub expected_value: String,
}

impl ExactParameterEvaluationStrategy {
    pub fn builder() -> ExactParameterEvaluationStrategyBuilder {
        <ExactParameterEvaluationStrategyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExactParameterEvaluationStrategyBuilder {
    expected_value: Option<String>,
}

impl ExactParameterEvaluationStrategyBuilder {
    pub fn expected_value(mut self, value: impl Into<String>) -> Self {
        self.expected_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExactParameterEvaluationStrategy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_value`](ExactParameterEvaluationStrategyBuilder::expected_value)
    pub fn build(self) -> Result<ExactParameterEvaluationStrategy, BuildError> {
        Ok(ExactParameterEvaluationStrategy {
            expected_value: self
                .expected_value
                .ok_or_else(|| BuildError::missing_field("expected_value"))?,
        })
    }
}
