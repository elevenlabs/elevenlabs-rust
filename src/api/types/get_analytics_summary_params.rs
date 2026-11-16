pub use crate::prelude::*;

/// Get a summary of key business analytics for a time period.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAnalyticsSummaryParams {}

impl GetAnalyticsSummaryParams {
    pub fn builder() -> GetAnalyticsSummaryParamsBuilder {
        <GetAnalyticsSummaryParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAnalyticsSummaryParamsBuilder {}

impl GetAnalyticsSummaryParamsBuilder {
    /// Consumes the builder and constructs a [`GetAnalyticsSummaryParams`].
    pub fn build(self) -> Result<GetAnalyticsSummaryParams, BuildError> {
        Ok(GetAnalyticsSummaryParams {})
    }
}
