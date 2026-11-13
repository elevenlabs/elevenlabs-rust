pub use crate::prelude::*;

/// Request for Create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftsCreateRequest {
    /// The ID of the agent branch to use
    #[serde(skip)]
    pub branch_id: String,
    pub body: BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost,
}

impl DraftsCreateRequest {
    pub fn builder() -> DraftsCreateRequestBuilder {
        <DraftsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DraftsCreateRequestBuilder {
    branch_id: Option<String>,
    body: Option<BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost>,
}

impl DraftsCreateRequestBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DraftsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](DraftsCreateRequestBuilder::branch_id)
    /// - [`body`](DraftsCreateRequestBuilder::body)
    pub fn build(self) -> Result<DraftsCreateRequest, BuildError> {
        Ok(DraftsCreateRequest {
            branch_id: self
                .branch_id
                .ok_or_else(|| BuildError::missing_field("branch_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
