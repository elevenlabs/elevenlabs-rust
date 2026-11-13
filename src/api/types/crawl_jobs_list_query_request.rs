pub use crate::prelude::*;

/// Query parameters for List Ongoing And Recent Crawl Jobs Created By A User
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CrawlJobsListQueryRequest {
    /// Ids of additional crawl jobs to retrieve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_job_ids: Option<Vec<String>>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl CrawlJobsListQueryRequest {
    pub fn builder() -> CrawlJobsListQueryRequestBuilder {
        <CrawlJobsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CrawlJobsListQueryRequestBuilder {
    include_job_ids: Option<Vec<String>>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl CrawlJobsListQueryRequestBuilder {
    pub fn include_job_ids(mut self, value: Vec<String>) -> Self {
        self.include_job_ids = Some(value);
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

    /// Consumes the builder and constructs a [`CrawlJobsListQueryRequest`].
    pub fn build(self) -> Result<CrawlJobsListQueryRequest, BuildError> {
        Ok(CrawlJobsListQueryRequest {
            include_job_ids: self.include_job_ids,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
