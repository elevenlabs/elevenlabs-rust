pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InitialiseContextGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_length_schedule: Option<Vec<i64>>,
}

impl InitialiseContextGenerationConfig {
    pub fn builder() -> InitialiseContextGenerationConfigBuilder {
        <InitialiseContextGenerationConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialiseContextGenerationConfigBuilder {
    chunk_length_schedule: Option<Vec<i64>>,
}

impl InitialiseContextGenerationConfigBuilder {
    pub fn chunk_length_schedule(mut self, value: Vec<i64>) -> Self {
        self.chunk_length_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InitialiseContextGenerationConfig`].
    pub fn build(self) -> Result<InitialiseContextGenerationConfig, BuildError> {
        Ok(InitialiseContextGenerationConfig {
            chunk_length_schedule: self.chunk_length_schedule,
        })
    }
}
