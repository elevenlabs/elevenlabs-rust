pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateEnvironmentVariableRequest {
    /// Values to replace. Set to null to remove the value for an environment (except 'production').
    #[serde(default)]
    pub values: std::collections::HashMap<String, Option<UpdateEnvironmentVariableRequestValues>>,
}

impl UpdateEnvironmentVariableRequest {
    pub fn builder() -> UpdateEnvironmentVariableRequestBuilder {
        <UpdateEnvironmentVariableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEnvironmentVariableRequestBuilder {
    values:
        Option<std::collections::HashMap<String, Option<UpdateEnvironmentVariableRequestValues>>>,
}

impl UpdateEnvironmentVariableRequestBuilder {
    pub fn values(
        mut self,
        value: std::collections::HashMap<String, Option<UpdateEnvironmentVariableRequestValues>>,
    ) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEnvironmentVariableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`values`](UpdateEnvironmentVariableRequestBuilder::values)
    pub fn build(self) -> Result<UpdateEnvironmentVariableRequest, BuildError> {
        Ok(UpdateEnvironmentVariableRequest {
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
