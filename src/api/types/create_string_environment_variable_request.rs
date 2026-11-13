pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStringEnvironmentVariableRequest {
    /// Unique label for the environment variable.
    #[serde(default)]
    pub label: String,
    /// Environment-specific values. Must include 'production' key.
    #[serde(default)]
    pub values: std::collections::HashMap<String, String>,
}

impl CreateStringEnvironmentVariableRequest {
    pub fn builder() -> CreateStringEnvironmentVariableRequestBuilder {
        <CreateStringEnvironmentVariableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStringEnvironmentVariableRequestBuilder {
    label: Option<String>,
    values: Option<std::collections::HashMap<String, String>>,
}

impl CreateStringEnvironmentVariableRequestBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn values(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateStringEnvironmentVariableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](CreateStringEnvironmentVariableRequestBuilder::label)
    /// - [`values`](CreateStringEnvironmentVariableRequestBuilder::values)
    pub fn build(self) -> Result<CreateStringEnvironmentVariableRequest, BuildError> {
        Ok(CreateStringEnvironmentVariableRequest {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
