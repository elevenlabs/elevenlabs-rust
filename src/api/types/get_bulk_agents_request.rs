pub use crate::prelude::*;

/// Request for GetBulkAgents (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetBulkAgentsRequest {
    /// Type of dependent agents to return.
    #[serde(skip)]
    pub dependent_type: Option<KnowledgeBaseDependentType>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip)]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip)]
    pub cursor: Option<String>,
    pub body: BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost,
}

impl GetBulkAgentsRequest {
    pub fn builder() -> GetBulkAgentsRequestBuilder {
        <GetBulkAgentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBulkAgentsRequestBuilder {
    dependent_type: Option<KnowledgeBaseDependentType>,
    page_size: Option<i64>,
    cursor: Option<String>,
    body:
        Option<BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost>,
}

impl GetBulkAgentsRequestBuilder {
    pub fn dependent_type(mut self, value: KnowledgeBaseDependentType) -> Self {
        self.dependent_type = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn body(
        mut self,
        value: BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost,
    ) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBulkAgentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](GetBulkAgentsRequestBuilder::body)
    pub fn build(self) -> Result<GetBulkAgentsRequest, BuildError> {
        Ok(GetBulkAgentsRequest {
            dependent_type: self.dependent_type,
            page_size: self.page_size,
            cursor: self.cursor,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
