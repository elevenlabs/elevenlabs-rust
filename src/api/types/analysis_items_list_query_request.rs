pub use crate::prelude::*;

/// Query parameters for List Analysis Items
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisItemsListQueryRequest {
    /// If specified, only analysis items whose names start with this string are returned, ordered by relevance rather than sort_direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// If specified, only analysis items of this kind are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AnalysisItemKind>,
    /// If specified, only analysis items with these IDs are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_item_ids: Option<Vec<String>>,
    /// Filter by creator user ID. Use '@me' to refer to the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AnalysisItemsListQueryRequest {
    pub fn builder() -> AnalysisItemsListQueryRequestBuilder {
        <AnalysisItemsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemsListQueryRequestBuilder {
    search: Option<String>,
    page_size: Option<i64>,
    kind: Option<AnalysisItemKind>,
    analysis_item_ids: Option<Vec<String>>,
    created_by_user_id: Option<String>,
    sort_direction: Option<SortDirection>,
    cursor: Option<String>,
}

impl AnalysisItemsListQueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn kind(mut self, value: AnalysisItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn analysis_item_ids(mut self, value: Vec<String>) -> Self {
        self.analysis_item_ids = Some(value);
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemsListQueryRequest`].
    pub fn build(self) -> Result<AnalysisItemsListQueryRequest, BuildError> {
        Ok(AnalysisItemsListQueryRequest {
            search: self.search,
            page_size: self.page_size,
            kind: self.kind,
            analysis_item_ids: self.analysis_item_ids,
            created_by_user_id: self.created_by_user_id,
            sort_direction: self.sort_direction,
            cursor: self.cursor,
        })
    }
}
