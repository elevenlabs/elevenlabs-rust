pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LlmParameterEvaluationStrategy {
    /// A description of the evaluation strategy to use for the test.
    #[serde(default)]
    pub description: String,
}

impl LlmParameterEvaluationStrategy {
    pub fn builder() -> LlmParameterEvaluationStrategyBuilder {
        <LlmParameterEvaluationStrategyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmParameterEvaluationStrategyBuilder {
    description: Option<String>,
}

impl LlmParameterEvaluationStrategyBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LlmParameterEvaluationStrategy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](LlmParameterEvaluationStrategyBuilder::description)
    pub fn build(self) -> Result<LlmParameterEvaluationStrategy, BuildError> {
        Ok(LlmParameterEvaluationStrategy {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
