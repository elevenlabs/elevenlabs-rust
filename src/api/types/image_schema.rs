pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ImageSchema {
    pub fn builder() -> ImageSchemaBuilder {
        <ImageSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl ImageSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImageSchema`].
    pub fn build(self) -> Result<ImageSchema, BuildError> {
        Ok(ImageSchema {
            title: self.title,
            description: self.description,
        })
    }
}
