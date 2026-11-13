pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSecretEnvironmentVariableRequest {
    /// Unique label for the environment variable.
    #[serde(default)]
    pub label: String,
    /// Environment-specific secret references. Must include 'production' key.
    #[serde(default)]
    pub values: std::collections::HashMap<String, EnvironmentVariableSecretValueRequest>,
}

impl CreateSecretEnvironmentVariableRequest {
    pub fn builder() -> CreateSecretEnvironmentVariableRequestBuilder {
        <CreateSecretEnvironmentVariableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSecretEnvironmentVariableRequestBuilder {
    label: Option<String>,
    values: Option<std::collections::HashMap<String, EnvironmentVariableSecretValueRequest>>,
}

impl CreateSecretEnvironmentVariableRequestBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn values(
        mut self,
        value: std::collections::HashMap<String, EnvironmentVariableSecretValueRequest>,
    ) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSecretEnvironmentVariableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](CreateSecretEnvironmentVariableRequestBuilder::label)
    /// - [`values`](CreateSecretEnvironmentVariableRequestBuilder::values)
    pub fn build(self) -> Result<CreateSecretEnvironmentVariableRequest, BuildError> {
        Ok(CreateSecretEnvironmentVariableRequest {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
