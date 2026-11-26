pub use crate::prelude::*;

/// Query parameters for List Ticket Tags
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TagsListQueryRequest2 {
    /// Only return ticket tags with these IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Whether to include archived ticket tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// Number of ticket tags to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl TagsListQueryRequest2 {
    pub fn builder() -> TagsListQueryRequest2Builder {
        <TagsListQueryRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TagsListQueryRequest2Builder {
    ids: Option<Vec<String>>,
    include_archived: Option<bool>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl TagsListQueryRequest2Builder {
    pub fn ids(mut self, value: Vec<String>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
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

    /// Consumes the builder and constructs a [`TagsListQueryRequest2`].
    pub fn build(self) -> Result<TagsListQueryRequest2, BuildError> {
        Ok(TagsListQueryRequest2 {
            ids: self.ids,
            include_archived: self.include_archived,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
