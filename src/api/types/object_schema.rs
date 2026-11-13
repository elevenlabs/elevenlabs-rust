pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ObjectSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, ContentSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl ObjectSchema {
    pub fn builder() -> ObjectSchemaBuilder {
        <ObjectSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObjectSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
    properties: Option<std::collections::HashMap<String, ContentSchema>>,
    required: Option<Vec<String>>,
}

impl ObjectSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn properties(mut self, value: std::collections::HashMap<String, ContentSchema>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ObjectSchema`].
    pub fn build(self) -> Result<ObjectSchema, BuildError> {
        Ok(ObjectSchema {
            title: self.title,
            description: self.description,
            properties: self.properties,
            required: self.required,
        })
    }
}
