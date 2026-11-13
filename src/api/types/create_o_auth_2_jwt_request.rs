pub use crate::prelude::*;

/// Request model for creating OAuth2 JWT auth connections - inherits common settings and includes sensitive fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOAuth2JwtRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    /// JWT signing algorithm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<CreateOAuth2JwtRequestAlgorithm>,
    /// Key ID (kid) for JWT header - useful for key rotation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// JWT issuer (iss claim)
    #[serde(default)]
    pub issuer: String,
    /// JWT audience (aud claim)
    #[serde(default)]
    pub audience: String,
    /// JWT subject (sub claim)
    #[serde(default)]
    pub subject: String,
    /// Token expiration time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub expiration_seconds: Option<i64>,
    /// Additional custom claims to include in the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::HashMap<String, String>>,
    /// Additional JWT header parameters (e.g. x5t). 'alg' and 'kid' are set from algorithm and key_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_headers: Option<std::collections::HashMap<String, String>>,
    /// Token endpoint URL for exchanging JWT for access token
    #[serde(default)]
    pub token_url: String,
    /// OAuth2 scopes to request when exchanging JWT for access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Token field to extract from the token endpoint response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>,
    /// How the JWT is used at the token endpoint. 'authorization_grant' sends it as a jwt-bearer grant. 'client_authentication' sends it as a client_assertion on a client_credentials request, with the subject as client_id and scopes in the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_usage: Option<CreateOAuth2JwtRequestJwtUsage>,
    #[serde(default)]
    pub secret_key: String,
}

impl CreateOAuth2JwtRequest {
    pub fn builder() -> CreateOAuth2JwtRequestBuilder {
        <CreateOAuth2JwtRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOAuth2JwtRequestBuilder {
    name: Option<String>,
    provider: Option<String>,
    algorithm: Option<CreateOAuth2JwtRequestAlgorithm>,
    key_id: Option<String>,
    issuer: Option<String>,
    audience: Option<String>,
    subject: Option<String>,
    expiration_seconds: Option<i64>,
    extra_params: Option<std::collections::HashMap<String, String>>,
    extra_headers: Option<std::collections::HashMap<String, String>>,
    token_url: Option<String>,
    scopes: Option<Vec<String>>,
    token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>,
    jwt_usage: Option<CreateOAuth2JwtRequestJwtUsage>,
    secret_key: Option<String>,
}

impl CreateOAuth2JwtRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn algorithm(mut self, value: CreateOAuth2JwtRequestAlgorithm) -> Self {
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

    pub fn extra_headers(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.extra_headers = Some(value);
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

    pub fn token_response_field(mut self, value: CreateOAuth2JwtRequestTokenResponseField) -> Self {
        self.token_response_field = Some(value);
        self
    }

    pub fn jwt_usage(mut self, value: CreateOAuth2JwtRequestJwtUsage) -> Self {
        self.jwt_usage = Some(value);
        self
    }

    pub fn secret_key(mut self, value: impl Into<String>) -> Self {
        self.secret_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOAuth2JwtRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateOAuth2JwtRequestBuilder::name)
    /// - [`provider`](CreateOAuth2JwtRequestBuilder::provider)
    /// - [`issuer`](CreateOAuth2JwtRequestBuilder::issuer)
    /// - [`audience`](CreateOAuth2JwtRequestBuilder::audience)
    /// - [`subject`](CreateOAuth2JwtRequestBuilder::subject)
    /// - [`token_url`](CreateOAuth2JwtRequestBuilder::token_url)
    /// - [`secret_key`](CreateOAuth2JwtRequestBuilder::secret_key)
    pub fn build(self) -> Result<CreateOAuth2JwtRequest, BuildError> {
        Ok(CreateOAuth2JwtRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            algorithm: self.algorithm,
            key_id: self.key_id,
            issuer: self
                .issuer
                .ok_or_else(|| BuildError::missing_field("issuer"))?,
            audience: self
                .audience
                .ok_or_else(|| BuildError::missing_field("audience"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            expiration_seconds: self.expiration_seconds,
            extra_params: self.extra_params,
            extra_headers: self.extra_headers,
            token_url: self
                .token_url
                .ok_or_else(|| BuildError::missing_field("token_url"))?,
            scopes: self.scopes,
            token_response_field: self.token_response_field,
            jwt_usage: self.jwt_usage,
            secret_key: self
                .secret_key
                .ok_or_else(|| BuildError::missing_field("secret_key"))?,
        })
    }
}
