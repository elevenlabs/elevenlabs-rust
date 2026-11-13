pub use crate::prelude::*;

/// Query parameters for Get Template
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplatesGetQueryRequest {
    /// How many of each template's published versions to return, newest first. `has_more_versions` tells you when a template has more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions_per_template: Option<i64>,
}

impl TemplatesGetQueryRequest {
    pub fn builder() -> TemplatesGetQueryRequestBuilder {
        <TemplatesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplatesGetQueryRequestBuilder {
    versions_per_template: Option<i64>,
}

impl TemplatesGetQueryRequestBuilder {
    pub fn versions_per_template(mut self, value: i64) -> Self {
        self.versions_per_template = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplatesGetQueryRequest`].
    pub fn build(self) -> Result<TemplatesGetQueryRequest, BuildError> {
        Ok(TemplatesGetQueryRequest {
            versions_per_template: self.versions_per_template,
        })
    }
}
