pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooleanSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl BooleanSchema {
    pub fn builder() -> BooleanSchemaBuilder {
        <BooleanSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooleanSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl BooleanSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BooleanSchema`].
    pub fn build(self) -> Result<BooleanSchema, BuildError> {
        Ok(BooleanSchema {
            title: self.title,
            description: self.description,
        })
    }
}
