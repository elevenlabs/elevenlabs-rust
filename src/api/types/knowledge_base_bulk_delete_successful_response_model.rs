pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseBulkDeleteSuccessfulResponseModel {
    pub data: KnowledgeBaseDeletedResponseModel,
}

impl KnowledgeBaseBulkDeleteSuccessfulResponseModel {
    pub fn builder() -> KnowledgeBaseBulkDeleteSuccessfulResponseModelBuilder {
        <KnowledgeBaseBulkDeleteSuccessfulResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseBulkDeleteSuccessfulResponseModelBuilder {
    data: Option<KnowledgeBaseDeletedResponseModel>,
}

impl KnowledgeBaseBulkDeleteSuccessfulResponseModelBuilder {
    pub fn data(mut self, value: KnowledgeBaseDeletedResponseModel) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseBulkDeleteSuccessfulResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](KnowledgeBaseBulkDeleteSuccessfulResponseModelBuilder::data)
    pub fn build(self) -> Result<KnowledgeBaseBulkDeleteSuccessfulResponseModel, BuildError> {
        Ok(KnowledgeBaseBulkDeleteSuccessfulResponseModel {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
