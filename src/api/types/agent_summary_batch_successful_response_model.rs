pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSummaryBatchSuccessfulResponseModel {
    pub data: AgentSummaryResponseModel,
}

impl AgentSummaryBatchSuccessfulResponseModel {
    pub fn builder() -> AgentSummaryBatchSuccessfulResponseModelBuilder {
        <AgentSummaryBatchSuccessfulResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSummaryBatchSuccessfulResponseModelBuilder {
    data: Option<AgentSummaryResponseModel>,
}

impl AgentSummaryBatchSuccessfulResponseModelBuilder {
    pub fn data(mut self, value: AgentSummaryResponseModel) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSummaryBatchSuccessfulResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AgentSummaryBatchSuccessfulResponseModelBuilder::data)
    pub fn build(self) -> Result<AgentSummaryBatchSuccessfulResponseModel, BuildError> {
        Ok(AgentSummaryBatchSuccessfulResponseModel {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
