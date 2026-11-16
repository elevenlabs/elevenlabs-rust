pub use crate::prelude::*;

/// Query parameters for List Agent Deployments
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeploymentsListQueryRequest {
    /// Page number, starting at 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// How many results at most should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl DeploymentsListQueryRequest {
    pub fn builder() -> DeploymentsListQueryRequestBuilder {
        <DeploymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentsListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl DeploymentsListQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentsListQueryRequest`].
    pub fn build(self) -> Result<DeploymentsListQueryRequest, BuildError> {
        Ok(DeploymentsListQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
