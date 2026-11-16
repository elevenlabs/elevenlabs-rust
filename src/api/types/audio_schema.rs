pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AudioSchema {
    pub fn builder() -> AudioSchemaBuilder {
        <AudioSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl AudioSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioSchema`].
    pub fn build(self) -> Result<AudioSchema, BuildError> {
        Ok(AudioSchema {
            title: self.title,
            description: self.description,
        })
    }
}
