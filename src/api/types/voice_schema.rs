pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VoiceSchema {
    pub fn builder() -> VoiceSchemaBuilder {
        <VoiceSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl VoiceSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceSchema`].
    pub fn build(self) -> Result<VoiceSchema, BuildError> {
        Ok(VoiceSchema {
            title: self.title,
            description: self.description,
        })
    }
}
