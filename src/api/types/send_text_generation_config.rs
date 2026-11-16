pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTextGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_length_schedule: Option<Vec<i64>>,
}

impl SendTextGenerationConfig {
    pub fn builder() -> SendTextGenerationConfigBuilder {
        <SendTextGenerationConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTextGenerationConfigBuilder {
    chunk_length_schedule: Option<Vec<i64>>,
}

impl SendTextGenerationConfigBuilder {
    pub fn chunk_length_schedule(mut self, value: Vec<i64>) -> Self {
        self.chunk_length_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTextGenerationConfig`].
    pub fn build(self) -> Result<SendTextGenerationConfig, BuildError> {
        Ok(SendTextGenerationConfig {
            chunk_length_schedule: self.chunk_length_schedule,
        })
    }
}
