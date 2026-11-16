pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MatchAnythingParameterEvaluationStrategy {}

impl MatchAnythingParameterEvaluationStrategy {
    pub fn builder() -> MatchAnythingParameterEvaluationStrategyBuilder {
        <MatchAnythingParameterEvaluationStrategyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatchAnythingParameterEvaluationStrategyBuilder {}

impl MatchAnythingParameterEvaluationStrategyBuilder {
    /// Consumes the builder and constructs a [`MatchAnythingParameterEvaluationStrategy`].
    pub fn build(self) -> Result<MatchAnythingParameterEvaluationStrategy, BuildError> {
        Ok(MatchAnythingParameterEvaluationStrategy {})
    }
}
