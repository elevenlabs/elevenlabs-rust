pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateOAuth2ClientCredsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_in_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

impl UpdateOAuth2ClientCredsRequest {
    pub fn builder() -> UpdateOAuth2ClientCredsRequestBuilder {
        <UpdateOAuth2ClientCredsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOAuth2ClientCredsRequestBuilder {
    provider: Option<String>,
    client_id: Option<String>,
    scopes: Option<Vec<String>>,
    extra_params: Option<std::collections::HashMap<String, String>>,
    basic_auth_in_header: Option<bool>,
    client_secret: Option<String>,
    custom_headers: Option<std::collections::HashMap<String, String>>,
}

impl UpdateOAuth2ClientCredsRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateOAuth2ClientCredsRequest`].
    pub fn build(self) -> Result<UpdateOAuth2ClientCredsRequest, BuildError> {
        Ok(UpdateOAuth2ClientCredsRequest {
            provider: self.provider,
            client_id: self.client_id,
            scopes: self.scopes,
            extra_params: self.extra_params,
            basic_auth_in_header: self.basic_auth_in_header,
            client_secret: self.client_secret,
            custom_headers: self.custom_headers,
        })
    }
}
