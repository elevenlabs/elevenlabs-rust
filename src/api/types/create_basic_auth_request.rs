pub use crate::prelude::*;

/// Request model for creating Basic Auth connections - inherits common settings and includes sensitive fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBasicAuthRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}

impl CreateBasicAuthRequest {
    pub fn builder() -> CreateBasicAuthRequestBuilder {
        <CreateBasicAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBasicAuthRequestBuilder {
    name: Option<String>,
    provider: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl CreateBasicAuthRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBasicAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateBasicAuthRequestBuilder::name)
    /// - [`provider`](CreateBasicAuthRequestBuilder::provider)
    /// - [`username`](CreateBasicAuthRequestBuilder::username)
    /// - [`password`](CreateBasicAuthRequestBuilder::password)
    pub fn build(self) -> Result<CreateBasicAuthRequest, BuildError> {
        Ok(CreateBasicAuthRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
            password: self
                .password
                .ok_or_else(|| BuildError::missing_field("password"))?,
        })
    }
}
