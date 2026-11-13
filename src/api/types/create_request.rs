pub use crate::prelude::*;

/// Request for Create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequest {
    /// Deprecated: all agents are versioned. This parameter is ignored.
    #[serde(skip)]
    pub enable_versioning: Option<bool>,
    pub body: BodyCreateAgentV1ConvaiAgentsCreatePost,
}

impl CreateRequest {
    pub fn builder() -> CreateRequestBuilder {
        <CreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestBuilder {
    enable_versioning: Option<bool>,
    body: Option<BodyCreateAgentV1ConvaiAgentsCreatePost>,
}

impl CreateRequestBuilder {
    pub fn enable_versioning(mut self, value: bool) -> Self {
        self.enable_versioning = Some(value);
        self
    }

    pub fn body(mut self, value: BodyCreateAgentV1ConvaiAgentsCreatePost) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](CreateRequestBuilder::body)
    pub fn build(self) -> Result<CreateRequest, BuildError> {
        Ok(CreateRequest {
            enable_versioning: self.enable_versioning,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
