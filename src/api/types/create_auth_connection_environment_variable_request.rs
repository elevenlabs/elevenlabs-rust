pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAuthConnectionEnvironmentVariableRequest {
    /// Unique label for the environment variable.
    #[serde(default)]
    pub label: String,
    /// Environment-specific auth connection references. Must include 'production' key.
    #[serde(default)]
    pub values: std::collections::HashMap<String, EnvironmentVariableAuthConnectionValueRequest>,
}

impl CreateAuthConnectionEnvironmentVariableRequest {
    pub fn builder() -> CreateAuthConnectionEnvironmentVariableRequestBuilder {
        <CreateAuthConnectionEnvironmentVariableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAuthConnectionEnvironmentVariableRequestBuilder {
    label: Option<String>,
    values:
        Option<std::collections::HashMap<String, EnvironmentVariableAuthConnectionValueRequest>>,
}

impl CreateAuthConnectionEnvironmentVariableRequestBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn values(
        mut self,
        value: std::collections::HashMap<String, EnvironmentVariableAuthConnectionValueRequest>,
    ) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAuthConnectionEnvironmentVariableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](CreateAuthConnectionEnvironmentVariableRequestBuilder::label)
    /// - [`values`](CreateAuthConnectionEnvironmentVariableRequestBuilder::values)
    pub fn build(self) -> Result<CreateAuthConnectionEnvironmentVariableRequest, BuildError> {
        Ok(CreateAuthConnectionEnvironmentVariableRequest {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
