pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArraySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub items: Box<ContentSchema>,
}

impl ArraySchema {
    pub fn builder() -> ArraySchemaBuilder {
        <ArraySchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArraySchemaBuilder {
    title: Option<String>,
    description: Option<String>,
    items: Option<Box<ContentSchema>>,
}

impl ArraySchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn items(mut self, value: Box<ContentSchema>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArraySchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ArraySchemaBuilder::items)
    pub fn build(self) -> Result<ArraySchema, BuildError> {
        Ok(ArraySchema {
            title: self.title,
            description: self.description,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
