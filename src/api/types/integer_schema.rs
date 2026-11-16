pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntegerSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl IntegerSchema {
    pub fn builder() -> IntegerSchemaBuilder {
        <IntegerSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntegerSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl IntegerSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntegerSchema`].
    pub fn build(self) -> Result<IntegerSchema, BuildError> {
        Ok(IntegerSchema {
            title: self.title,
            description: self.description,
        })
    }
}
