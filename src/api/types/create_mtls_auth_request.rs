pub use crate::prelude::*;

/// Request model for creating mTLS auth connections.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateMtlsAuthRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub client_certificate: String,
    #[serde(default)]
    pub client_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_passphrase: Option<String>,
}

impl CreateMtlsAuthRequest {
    pub fn builder() -> CreateMtlsAuthRequestBuilder {
        <CreateMtlsAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMtlsAuthRequestBuilder {
    name: Option<String>,
    provider: Option<String>,
    client_certificate: Option<String>,
    client_key: Option<String>,
    ca_certificate: Option<String>,
    key_passphrase: Option<String>,
}

impl CreateMtlsAuthRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn client_certificate(mut self, value: impl Into<String>) -> Self {
        self.client_certificate = Some(value.into());
        self
    }

    pub fn client_key(mut self, value: impl Into<String>) -> Self {
        self.client_key = Some(value.into());
        self
    }

    pub fn ca_certificate(mut self, value: impl Into<String>) -> Self {
        self.ca_certificate = Some(value.into());
        self
    }

    pub fn key_passphrase(mut self, value: impl Into<String>) -> Self {
        self.key_passphrase = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMtlsAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateMtlsAuthRequestBuilder::name)
    /// - [`provider`](CreateMtlsAuthRequestBuilder::provider)
    /// - [`client_certificate`](CreateMtlsAuthRequestBuilder::client_certificate)
    /// - [`client_key`](CreateMtlsAuthRequestBuilder::client_key)
    pub fn build(self) -> Result<CreateMtlsAuthRequest, BuildError> {
        Ok(CreateMtlsAuthRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            client_certificate: self
                .client_certificate
                .ok_or_else(|| BuildError::missing_field("client_certificate"))?,
            client_key: self
                .client_key
                .ok_or_else(|| BuildError::missing_field("client_key"))?,
            ca_certificate: self.ca_certificate,
            key_passphrase: self.key_passphrase,
        })
    }
}
