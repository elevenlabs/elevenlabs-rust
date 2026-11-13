pub use crate::prelude::*;

/// One input or output port of a published template version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplatePort {
    /// The port id. Input ids are the keys of the `inputs` map on `POST /v1/flows/templates/{template_id}/runs`; output ids are the keys of the `outputs` map on the run response.
    #[serde(default)]
    pub id: String,
    pub content_schema: ContentSchema,
}

impl TemplatePort {
    pub fn builder() -> TemplatePortBuilder {
        <TemplatePortBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplatePortBuilder {
    id: Option<String>,
    content_schema: Option<ContentSchema>,
}

impl TemplatePortBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content_schema(mut self, value: ContentSchema) -> Self {
        self.content_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplatePort`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplatePortBuilder::id)
    /// - [`content_schema`](TemplatePortBuilder::content_schema)
    pub fn build(self) -> Result<TemplatePort, BuildError> {
        Ok(TemplatePort {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content_schema: self
                .content_schema
                .ok_or_else(|| BuildError::missing_field("content_schema"))?,
        })
    }
}
