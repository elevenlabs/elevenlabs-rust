pub use crate::prelude::*;

/// Query parameters for List Dubs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingListQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many dubs to return at maximum. Can not exceed 200, defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// What state the dub is currently in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_status: Option<ListDubbingStatus>,
    /// Filter by dubbing status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_statuses: Option<Vec<DubbingStatuses>>,
    /// Filter by dubbing model generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_models: Option<Vec<DubbingModels>>,
    /// Filter by target language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language_codes: Option<Vec<String>>,
    /// Filter by dubbing creation source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_sources: Option<Vec<CreationSources>>,
    /// Filters who created the resources being listed, whether it was the user running the request or someone else that shared the resource with them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_creator: Option<FilterByCreator>,
    /// The field to use for ordering results from this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
    /// The order direction to use for results from this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<OrderDirection>,
}

impl DubbingListQueryRequest {
    pub fn builder() -> DubbingListQueryRequestBuilder {
        <DubbingListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    dubbing_status: Option<ListDubbingStatus>,
    dubbing_statuses: Option<Vec<DubbingStatuses>>,
    dubbing_models: Option<Vec<DubbingModels>>,
    target_language_codes: Option<Vec<String>>,
    creation_sources: Option<Vec<CreationSources>>,
    filter_by_creator: Option<FilterByCreator>,
    order_by: Option<OrderBy>,
    order_direction: Option<OrderDirection>,
}

impl DubbingListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn dubbing_status(mut self, value: ListDubbingStatus) -> Self {
        self.dubbing_status = Some(value);
        self
    }

    pub fn dubbing_statuses(mut self, value: Vec<DubbingStatuses>) -> Self {
        self.dubbing_statuses = Some(value);
        self
    }

    pub fn dubbing_models(mut self, value: Vec<DubbingModels>) -> Self {
        self.dubbing_models = Some(value);
        self
    }

    pub fn target_language_codes(mut self, value: Vec<String>) -> Self {
        self.target_language_codes = Some(value);
        self
    }

    pub fn creation_sources(mut self, value: Vec<CreationSources>) -> Self {
        self.creation_sources = Some(value);
        self
    }

    pub fn filter_by_creator(mut self, value: FilterByCreator) -> Self {
        self.filter_by_creator = Some(value);
        self
    }

    pub fn order_by(mut self, value: OrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    pub fn order_direction(mut self, value: OrderDirection) -> Self {
        self.order_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingListQueryRequest`].
    pub fn build(self) -> Result<DubbingListQueryRequest, BuildError> {
        Ok(DubbingListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            dubbing_status: self.dubbing_status,
            dubbing_statuses: self.dubbing_statuses,
            dubbing_models: self.dubbing_models,
            target_language_codes: self.target_language_codes,
            creation_sources: self.creation_sources,
            filter_by_creator: self.filter_by_creator,
            order_by: self.order_by,
            order_direction: self.order_direction,
        })
    }
}
