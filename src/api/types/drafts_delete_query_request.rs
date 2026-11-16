pub use crate::prelude::*;

/// Query parameters for Delete Agent Draft
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftsDeleteQueryRequest {
    /// The ID of the agent branch to use
    pub branch_id: String,
}

impl DraftsDeleteQueryRequest {
    pub fn builder() -> DraftsDeleteQueryRequestBuilder {
        <DraftsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DraftsDeleteQueryRequestBuilder {
    branch_id: Option<String>,
}

impl DraftsDeleteQueryRequestBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DraftsDeleteQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](DraftsDeleteQueryRequestBuilder::branch_id)
    pub fn build(self) -> Result<DraftsDeleteQueryRequest, BuildError> {
        Ok(DraftsDeleteQueryRequest {
            branch_id: self
                .branch_id
                .ok_or_else(|| BuildError::missing_field("branch_id"))?,
        })
    }
}
