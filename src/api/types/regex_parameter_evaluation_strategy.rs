pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegexParameterEvaluationStrategy {
    /// A regex pattern to match the agent's response against.
    #[serde(default)]
    pub pattern: String,
}

impl RegexParameterEvaluationStrategy {
    pub fn builder() -> RegexParameterEvaluationStrategyBuilder {
        <RegexParameterEvaluationStrategyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegexParameterEvaluationStrategyBuilder {
    pattern: Option<String>,
}

impl RegexParameterEvaluationStrategyBuilder {
    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegexParameterEvaluationStrategy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pattern`](RegexParameterEvaluationStrategyBuilder::pattern)
    pub fn build(self) -> Result<RegexParameterEvaluationStrategy, BuildError> {
        Ok(RegexParameterEvaluationStrategy {
            pattern: self
                .pattern
                .ok_or_else(|| BuildError::missing_field("pattern"))?,
        })
    }
}
