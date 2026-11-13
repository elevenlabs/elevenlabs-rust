pub use crate::prelude::*;

/// Query parameters for List Conversation Tags
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TagsListQueryRequest {
    /// How many conversation tags to return. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl TagsListQueryRequest {
    pub fn builder() -> TagsListQueryRequestBuilder {
        <TagsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TagsListQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl TagsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TagsListQueryRequest`].
    pub fn build(self) -> Result<TagsListQueryRequest, BuildError> {
        Ok(TagsListQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
