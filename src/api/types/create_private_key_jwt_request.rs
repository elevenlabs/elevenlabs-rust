pub use crate::prelude::*;

/// Request model for creating Private Key JWT auth connections - inherits common settings and includes sensitive fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePrivateKeyJwtRequest {
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
    #[serde(default)]
    pub secret_key: String,
}

impl CreatePrivateKeyJwtRequest {
    pub fn builder() -> CreatePrivateKeyJwtRequestBuilder {
        <CreatePrivateKeyJwtRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePrivateKeyJwtRequestBuilder {
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
    secret_key: Option<String>,
}

impl CreatePrivateKeyJwtRequestBuilder {
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

    pub fn secret_key(mut self, value: impl Into<String>) -> Self {
        self.secret_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePrivateKeyJwtRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreatePrivateKeyJwtRequestBuilder::name)
    /// - [`provider`](CreatePrivateKeyJwtRequestBuilder::provider)
    /// - [`issuer`](CreatePrivateKeyJwtRequestBuilder::issuer)
    /// - [`audience`](CreatePrivateKeyJwtRequestBuilder::audience)
    /// - [`subject`](CreatePrivateKeyJwtRequestBuilder::subject)
    /// - [`secret_key`](CreatePrivateKeyJwtRequestBuilder::secret_key)
    pub fn build(self) -> Result<CreatePrivateKeyJwtRequest, BuildError> {
        Ok(CreatePrivateKeyJwtRequest {
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
            secret_key: self
                .secret_key
                .ok_or_else(|| BuildError::missing_field("secret_key"))?,
        })
    }
}
