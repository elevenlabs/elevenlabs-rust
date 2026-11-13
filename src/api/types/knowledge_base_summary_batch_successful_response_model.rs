pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeBaseSummaryBatchSuccessfulResponseModel {
    pub data: Data,
}

impl KnowledgeBaseSummaryBatchSuccessfulResponseModel {
    pub fn builder() -> KnowledgeBaseSummaryBatchSuccessfulResponseModelBuilder {
        <KnowledgeBaseSummaryBatchSuccessfulResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseSummaryBatchSuccessfulResponseModelBuilder {
    data: Option<Data>,
}

impl KnowledgeBaseSummaryBatchSuccessfulResponseModelBuilder {
    pub fn data(mut self, value: Data) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseSummaryBatchSuccessfulResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](KnowledgeBaseSummaryBatchSuccessfulResponseModelBuilder::data)
    pub fn build(self) -> Result<KnowledgeBaseSummaryBatchSuccessfulResponseModel, BuildError> {
        Ok(KnowledgeBaseSummaryBatchSuccessfulResponseModel {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
