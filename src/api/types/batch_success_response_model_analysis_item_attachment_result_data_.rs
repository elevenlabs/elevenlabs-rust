pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BatchSuccessResponseModelAnalysisItemAttachmentResultData_ {
    pub data: AnalysisItemAttachmentResultData,
}

impl BatchSuccessResponseModelAnalysisItemAttachmentResultData_ {
    pub fn builder() -> BatchSuccessResponseModelAnalysisItemAttachmentResultData_Builder {
        <BatchSuccessResponseModelAnalysisItemAttachmentResultData_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchSuccessResponseModelAnalysisItemAttachmentResultData_Builder {
    data: Option<AnalysisItemAttachmentResultData>,
}

impl BatchSuccessResponseModelAnalysisItemAttachmentResultData_Builder {
    pub fn data(mut self, value: AnalysisItemAttachmentResultData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchSuccessResponseModelAnalysisItemAttachmentResultData_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](BatchSuccessResponseModelAnalysisItemAttachmentResultData_Builder::data)
    pub fn build(
        self,
    ) -> Result<BatchSuccessResponseModelAnalysisItemAttachmentResultData_, BuildError> {
        Ok(BatchSuccessResponseModelAnalysisItemAttachmentResultData_ {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
