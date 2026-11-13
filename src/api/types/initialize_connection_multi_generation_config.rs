pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InitializeConnectionMultiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_length_schedule: Option<Vec<i64>>,
}

impl InitializeConnectionMultiGenerationConfig {
    pub fn builder() -> InitializeConnectionMultiGenerationConfigBuilder {
        <InitializeConnectionMultiGenerationConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionMultiGenerationConfigBuilder {
    chunk_length_schedule: Option<Vec<i64>>,
}

impl InitializeConnectionMultiGenerationConfigBuilder {
    pub fn chunk_length_schedule(mut self, value: Vec<i64>) -> Self {
        self.chunk_length_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InitializeConnectionMultiGenerationConfig`].
    pub fn build(self) -> Result<InitializeConnectionMultiGenerationConfig, BuildError> {
        Ok(InitializeConnectionMultiGenerationConfig {
            chunk_length_schedule: self.chunk_length_schedule,
        })
    }
}
