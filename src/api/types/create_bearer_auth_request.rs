pub use crate::prelude::*;

/// Request model for creating Bearer Auth connections - inherits common settings and includes sensitive fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBearerAuthRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub token: String,
}

impl CreateBearerAuthRequest {
    pub fn builder() -> CreateBearerAuthRequestBuilder {
        <CreateBearerAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBearerAuthRequestBuilder {
    name: Option<String>,
    provider: Option<String>,
    token: Option<String>,
}

impl CreateBearerAuthRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBearerAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateBearerAuthRequestBuilder::name)
    /// - [`provider`](CreateBearerAuthRequestBuilder::provider)
    /// - [`token`](CreateBearerAuthRequestBuilder::token)
    pub fn build(self) -> Result<CreateBearerAuthRequest, BuildError> {
        Ok(CreateBearerAuthRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
