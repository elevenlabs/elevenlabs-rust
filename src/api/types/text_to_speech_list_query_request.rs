pub use crate::prelude::*;

/// Query parameters for List Speech Generations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToSpeechListQueryRequest {
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

impl TextToSpeechListQueryRequest {
    pub fn builder() -> TextToSpeechListQueryRequestBuilder {
        <TextToSpeechListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToSpeechListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<ListStatusQueryParameter1>,
    model_id: Option<String>,
}

impl TextToSpeechListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`TextToSpeechListQueryRequest`].
    pub fn build(self) -> Result<TextToSpeechListQueryRequest, BuildError> {
        Ok(TextToSpeechListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
            model_id: self.model_id,
        })
    }
}
