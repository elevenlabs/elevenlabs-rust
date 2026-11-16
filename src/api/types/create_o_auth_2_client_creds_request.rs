pub use crate::prelude::*;

/// Request model for creating auth connections - inherits common settings and includes sensitive fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOAuth2ClientCredsRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::HashMap<String, String>>,
    /// If True, send client credentials in Authorization header as Basic Auth instead of request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_in_header: Option<bool>,
    #[serde(default)]
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

impl CreateOAuth2ClientCredsRequest {
    pub fn builder() -> CreateOAuth2ClientCredsRequestBuilder {
        <CreateOAuth2ClientCredsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOAuth2ClientCredsRequestBuilder {
    name: Option<String>,
    provider: Option<String>,
    client_id: Option<String>,
    token_url: Option<String>,
    scopes: Option<Vec<String>>,
    extra_params: Option<std::collections::HashMap<String, String>>,
    basic_auth_in_header: Option<bool>,
    client_secret: Option<String>,
    custom_headers: Option<std::collections::HashMap<String, String>>,
}

impl CreateOAuth2ClientCredsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn token_url(mut self, value: impl Into<String>) -> Self {
        self.token_url = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn extra_params(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.extra_params = Some(value);
        self
    }

    pub fn basic_auth_in_header(mut self, value: bool) -> Self {
        self.basic_auth_in_header = Some(value);
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn custom_headers(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.custom_headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOAuth2ClientCredsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateOAuth2ClientCredsRequestBuilder::name)
    /// - [`provider`](CreateOAuth2ClientCredsRequestBuilder::provider)
    /// - [`client_id`](CreateOAuth2ClientCredsRequestBuilder::client_id)
    /// - [`token_url`](CreateOAuth2ClientCredsRequestBuilder::token_url)
    /// - [`client_secret`](CreateOAuth2ClientCredsRequestBuilder::client_secret)
    pub fn build(self) -> Result<CreateOAuth2ClientCredsRequest, BuildError> {
        Ok(CreateOAuth2ClientCredsRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            token_url: self
                .token_url
                .ok_or_else(|| BuildError::missing_field("token_url"))?,
            scopes: self.scopes,
            extra_params: self.extra_params,
            basic_auth_in_header: self.basic_auth_in_header,
            client_secret: self
                .client_secret
                .ok_or_else(|| BuildError::missing_field("client_secret"))?,
            custom_headers: self.custom_headers,
        })
    }
}
