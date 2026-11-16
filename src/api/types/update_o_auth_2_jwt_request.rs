pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateOAuth2JwtRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub expiration_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_usage: Option<UpdateOAuth2JwtRequestJwtUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_headers: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

impl UpdateOAuth2JwtRequest {
    pub fn builder() -> UpdateOAuth2JwtRequestBuilder {
        <UpdateOAuth2JwtRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOAuth2JwtRequestBuilder {
    provider: Option<String>,
    algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>,
    key_id: Option<String>,
    issuer: Option<String>,
    audience: Option<String>,
    subject: Option<String>,
    expiration_seconds: Option<i64>,
    extra_params: Option<std::collections::HashMap<String, String>>,
    scopes: Option<Vec<String>>,
    token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>,
    jwt_usage: Option<UpdateOAuth2JwtRequestJwtUsage>,
    extra_headers: Option<std::collections::HashMap<String, String>>,
    secret_key: Option<String>,
}

impl UpdateOAuth2JwtRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn algorithm(mut self, value: UpdateOAuth2JwtRequestAlgorithm) -> Self {
        self.algorithm = Some(value);
        self
    }

    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.key_id = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn audience(mut self, value: impl Into<String>) -> Self {
        self.audience = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn expiration_seconds(mut self, value: i64) -> Self {
        self.expiration_seconds = Some(value);
        self
    }

    pub fn extra_params(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.extra_params = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn token_response_field(mut self, value: UpdateOAuth2JwtRequestTokenResponseField) -> Self {
        self.token_response_field = Some(value);
        self
    }

    pub fn jwt_usage(mut self, value: UpdateOAuth2JwtRequestJwtUsage) -> Self {
        self.jwt_usage = Some(value);
        self
    }

    pub fn extra_headers(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.extra_headers = Some(value);
        self
    }

    pub fn secret_key(mut self, value: impl Into<String>) -> Self {
        self.secret_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateOAuth2JwtRequest`].
    pub fn build(self) -> Result<UpdateOAuth2JwtRequest, BuildError> {
        Ok(UpdateOAuth2JwtRequest {
            provider: self.provider,
            algorithm: self.algorithm,
            key_id: self.key_id,
            issuer: self.issuer,
            audience: self.audience,
            subject: self.subject,
            expiration_seconds: self.expiration_seconds,
            extra_params: self.extra_params,
            scopes: self.scopes,
            token_response_field: self.token_response_field,
            jwt_usage: self.jwt_usage,
            extra_headers: self.extra_headers,
            secret_key: self.secret_key,
        })
    }
}
