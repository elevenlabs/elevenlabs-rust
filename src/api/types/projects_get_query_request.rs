pub use crate::prelude::*;

/// Query parameters for Get Studio Project
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectsGetQueryRequest {
    /// The share ID of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
}

impl ProjectsGetQueryRequest {
    pub fn builder() -> ProjectsGetQueryRequestBuilder {
        <ProjectsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsGetQueryRequestBuilder {
    share_id: Option<String>,
}

impl ProjectsGetQueryRequestBuilder {
    pub fn share_id(mut self, value: impl Into<String>) -> Self {
        self.share_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProjectsGetQueryRequest`].
    pub fn build(self) -> Result<ProjectsGetQueryRequest, BuildError> {
        Ok(ProjectsGetQueryRequest {
            share_id: self.share_id,
        })
    }
}
