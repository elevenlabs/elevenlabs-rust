pub use crate::prelude::*;

/// Query parameters for List Image Generations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImageListQueryRequest {
    /// Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many generations to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only return generations with this lifecycle status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListStatusQueryParameter1>,
    /// Only return generations of this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

impl ImageListQueryRequest {
    pub fn builder() -> ImageListQueryRequestBuilder {
        <ImageListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<ListStatusQueryParameter1>,
    model_id: Option<String>,
}

impl ImageListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: ListStatusQueryParameter1) -> Self {
        self.status = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImageListQueryRequest`].
    pub fn build(self) -> Result<ImageListQueryRequest, BuildError> {
        Ok(ImageListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
            model_id: self.model_id,
        })
    }
}
