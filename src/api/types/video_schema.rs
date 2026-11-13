pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VideoSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VideoSchema {
    pub fn builder() -> VideoSchemaBuilder {
        <VideoSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoSchemaBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl VideoSchemaBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VideoSchema`].
    pub fn build(self) -> Result<VideoSchema, BuildError> {
        Ok(VideoSchema {
            title: self.title,
            description: self.description,
        })
    }
}
