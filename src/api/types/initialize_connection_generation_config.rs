pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InitializeConnectionGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_length_schedule: Option<Vec<i64>>,
}

impl InitializeConnectionGenerationConfig {
    pub fn builder() -> InitializeConnectionGenerationConfigBuilder {
        <InitializeConnectionGenerationConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionGenerationConfigBuilder {
    chunk_length_schedule: Option<Vec<i64>>,
}

impl InitializeConnectionGenerationConfigBuilder {
    pub fn chunk_length_schedule(mut self, value: Vec<i64>) -> Self {
        self.chunk_length_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InitializeConnectionGenerationConfig`].
    pub fn build(self) -> Result<InitializeConnectionGenerationConfig, BuildError> {
        Ok(InitializeConnectionGenerationConfig {
            chunk_length_schedule: self.chunk_length_schedule,
        })
    }
}
