pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StringSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
}

impl StringSchema {
    pub fn builder() -> StringSchemaBuilder {
        <StringSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StringSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
    r#enum: Option<Vec<String>>,
}

impl StringSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn r#enum(mut self, value: Vec<String>) -> Self {
        self.r#enum = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StringSchema`].
    pub fn build(self) -> Result<StringSchema, BuildError> {
        Ok(StringSchema {
            title: self.title,
            description: self.description,
            r#enum: self.r#enum,
        })
    }
}
