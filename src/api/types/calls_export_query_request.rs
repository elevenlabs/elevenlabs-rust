pub use crate::prelude::*;

/// Query parameters for Export Outbound Campaign Calls
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CallsExportQueryRequest {
    /// Only export the first N calls; used to preview the columns. Omit to export every call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl CallsExportQueryRequest {
    pub fn builder() -> CallsExportQueryRequestBuilder {
        <CallsExportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CallsExportQueryRequestBuilder {
    limit: Option<i64>,
}

impl CallsExportQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CallsExportQueryRequest`].
    pub fn build(self) -> Result<CallsExportQueryRequest, BuildError> {
        Ok(CallsExportQueryRequest { limit: self.limit })
    }
}
