pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NumberSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<f64>>,
}

impl NumberSchema {
    pub fn builder() -> NumberSchemaBuilder {
        <NumberSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NumberSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
    r#enum: Option<Vec<f64>>,
}

impl NumberSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn r#enum(mut self, value: Vec<f64>) -> Self {
        self.r#enum = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NumberSchema`].
    pub fn build(self) -> Result<NumberSchema, BuildError> {
        Ok(NumberSchema {
            title: self.title,
            description: self.description,
            r#enum: self.r#enum,
        })
    }
}
