pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RagIndexBatchSuccessfulResponseModel {
    pub data: RagDocumentIndexResponseModel,
}

impl RagIndexBatchSuccessfulResponseModel {
    pub fn builder() -> RagIndexBatchSuccessfulResponseModelBuilder {
        <RagIndexBatchSuccessfulResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagIndexBatchSuccessfulResponseModelBuilder {
    data: Option<RagDocumentIndexResponseModel>,
}

impl RagIndexBatchSuccessfulResponseModelBuilder {
    pub fn data(mut self, value: RagDocumentIndexResponseModel) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagIndexBatchSuccessfulResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RagIndexBatchSuccessfulResponseModelBuilder::data)
    pub fn build(self) -> Result<RagIndexBatchSuccessfulResponseModel, BuildError> {
        Ok(RagIndexBatchSuccessfulResponseModel {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
