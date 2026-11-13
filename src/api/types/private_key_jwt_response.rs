pub use crate::prelude::*;

/// Response model for Private Key JWT auth connections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateKeyJwtResponse {
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
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<AuthConnectionDependencies>,
    /// Single status field shared by every auth type's stored credential.
    ///
    /// OAuth values (``REFRESH_FAILED``, ``REVOKED``) are written by the OAuth
    /// token-manager refresh path. ``CREDENTIAL_INVALID`` is written by the
    /// tool execution path when an upstream response matches a credential's
    /// ``failure_signatures`` entry (Bearer, Basic auth, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AuthConnectionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_updated_at: Option<String>,
}

impl PrivateKeyJwtResponse {
    pub fn builder() -> PrivateKeyJwtResponseBuilder {
        <PrivateKeyJwtResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrivateKeyJwtResponseBuilder {
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
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl PrivateKeyJwtResponseBuilder {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn used_by(mut self, value: AuthConnectionDependencies) -> Self {
        self.used_by = Some(value);
        self
    }

    pub fn status(mut self, value: AuthConnectionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_detail(mut self, value: impl Into<String>) -> Self {
        self.status_detail = Some(value.into());
        self
    }

    pub fn status_updated_at(mut self, value: impl Into<String>) -> Self {
        self.status_updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PrivateKeyJwtResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PrivateKeyJwtResponseBuilder::name)
    /// - [`provider`](PrivateKeyJwtResponseBuilder::provider)
    /// - [`issuer`](PrivateKeyJwtResponseBuilder::issuer)
    /// - [`audience`](PrivateKeyJwtResponseBuilder::audience)
    /// - [`subject`](PrivateKeyJwtResponseBuilder::subject)
    /// - [`id`](PrivateKeyJwtResponseBuilder::id)
    pub fn build(self) -> Result<PrivateKeyJwtResponse, BuildError> {
        Ok(PrivateKeyJwtResponse {
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
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
