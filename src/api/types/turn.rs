pub use crate::prelude::*;

/// Configuration for turn detection
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Turn {
    /// Configuration for soft timeout functionality. Provides immediate feedback during longer LLM responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_timeout_config: Option<TurnSoftTimeoutConfig>,
    /// What the agent says when it detects a noisy environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noisy_environment_message: Option<String>,
}

impl Turn {
    pub fn builder() -> TurnBuilder {
        <TurnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurnBuilder {
    soft_timeout_config: Option<TurnSoftTimeoutConfig>,
    noisy_environment_message: Option<String>,
}

impl TurnBuilder {
    pub fn soft_timeout_config(mut self, value: TurnSoftTimeoutConfig) -> Self {
        self.soft_timeout_config = Some(value);
        self
    }

    pub fn noisy_environment_message(mut self, value: impl Into<String>) -> Self {
        self.noisy_environment_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Turn`].
    pub fn build(self) -> Result<Turn, BuildError> {
        Ok(Turn {
            soft_timeout_config: self.soft_timeout_config,
            noisy_environment_message: self.noisy_environment_message,
        })
    }
}
