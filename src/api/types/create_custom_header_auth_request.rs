pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateCustomHeaderAuthRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    /// The name of the header to use for authentication (e.g., 'x-api-key')
    #[serde(default)]
    pub header_name: String,
    #[serde(default)]
    pub token: String,
}

impl CreateCustomHeaderAuthRequest {
    pub fn builder() -> CreateCustomHeaderAuthRequestBuilder {
        <CreateCustomHeaderAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCustomHeaderAuthRequestBuilder {
    name: Option<String>,
    provider: Option<String>,
    header_name: Option<String>,
    token: Option<String>,
}

impl CreateCustomHeaderAuthRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn header_name(mut self, value: impl Into<String>) -> Self {
        self.header_name = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCustomHeaderAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateCustomHeaderAuthRequestBuilder::name)
    /// - [`provider`](CreateCustomHeaderAuthRequestBuilder::provider)
    /// - [`header_name`](CreateCustomHeaderAuthRequestBuilder::header_name)
    /// - [`token`](CreateCustomHeaderAuthRequestBuilder::token)
    pub fn build(self) -> Result<CreateCustomHeaderAuthRequest, BuildError> {
        Ok(CreateCustomHeaderAuthRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            header_name: self
                .header_name
                .ok_or_else(|| BuildError::missing_field("header_name"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
