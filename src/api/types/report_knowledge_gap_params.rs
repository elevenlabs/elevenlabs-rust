pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportKnowledgeGapParams {}

impl ReportKnowledgeGapParams {
    pub fn builder() -> ReportKnowledgeGapParamsBuilder {
        <ReportKnowledgeGapParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportKnowledgeGapParamsBuilder {}

impl ReportKnowledgeGapParamsBuilder {
    /// Consumes the builder and constructs a [`ReportKnowledgeGapParams`].
    pub fn build(self) -> Result<ReportKnowledgeGapParams, BuildError> {
        Ok(ReportKnowledgeGapParams {})
    }
}
