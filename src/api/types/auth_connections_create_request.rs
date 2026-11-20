pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "auth_type")]
#[non_exhaustive]
pub enum AuthConnectionsCreateRequest {
    #[serde(rename = "oauth2_client_credentials")]
    Oauth2ClientCredentials {
        #[serde(flatten)]
        data: CreateOAuth2ClientCredsRequest,
    },

    #[serde(rename = "custom_header_auth")]
    CustomHeaderAuth {
        #[serde(flatten)]
        data: CreateCustomHeaderAuthRequest,
    },

    #[serde(rename = "basic_auth")]
    BasicAuth {
        #[serde(flatten)]
        data: CreateBasicAuthRequest,
    },

    #[serde(rename = "bearer_auth")]
    BearerAuth {
        #[serde(flatten)]
        data: CreateBearerAuthRequest,
    },

    #[serde(rename = "oauth2_jwt")]
    Oauth2Jwt {
        #[serde(flatten)]
        data: CreateOAuth2JwtRequest,
    },

    #[serde(rename = "private_key_jwt")]
    PrivateKeyJwt {
        #[serde(flatten)]
        data: CreatePrivateKeyJwtRequest,
    },

    #[serde(rename = "mtls")]
    Mtls {
        #[serde(flatten)]
        data: CreateMtlsAuthRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AuthConnectionsCreateRequest {
    pub fn is_oauth2_client_credentials(&self) -> bool {
        matches!(self, Self::Oauth2ClientCredentials { .. })
    }

    pub fn as_oauth2_client_credentials(&self) -> Option<&CreateOAuth2ClientCredsRequest> {
        match self {
            Self::Oauth2ClientCredentials { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_oauth2_client_credentials(self) -> Option<CreateOAuth2ClientCredsRequest> {
        match self {
            Self::Oauth2ClientCredentials { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_custom_header_auth(&self) -> bool {
        matches!(self, Self::CustomHeaderAuth { .. })
    }

    pub fn as_custom_header_auth(&self) -> Option<&CreateCustomHeaderAuthRequest> {
        match self {
            Self::CustomHeaderAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_custom_header_auth(self) -> Option<CreateCustomHeaderAuthRequest> {
        match self {
            Self::CustomHeaderAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_basic_auth(&self) -> bool {
        matches!(self, Self::BasicAuth { .. })
    }

    pub fn as_basic_auth(&self) -> Option<&CreateBasicAuthRequest> {
        match self {
            Self::BasicAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_basic_auth(self) -> Option<CreateBasicAuthRequest> {
        match self {
            Self::BasicAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bearer_auth(&self) -> bool {
        matches!(self, Self::BearerAuth { .. })
    }

    pub fn as_bearer_auth(&self) -> Option<&CreateBearerAuthRequest> {
        match self {
            Self::BearerAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bearer_auth(self) -> Option<CreateBearerAuthRequest> {
        match self {
            Self::BearerAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_oauth2_jwt(&self) -> bool {
        matches!(self, Self::Oauth2Jwt { .. })
    }

    pub fn as_oauth2_jwt(&self) -> Option<&CreateOAuth2JwtRequest> {
        match self {
            Self::Oauth2Jwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_oauth2_jwt(self) -> Option<CreateOAuth2JwtRequest> {
        match self {
            Self::Oauth2Jwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_private_key_jwt(&self) -> bool {
        matches!(self, Self::PrivateKeyJwt { .. })
    }

    pub fn as_private_key_jwt(&self) -> Option<&CreatePrivateKeyJwtRequest> {
        match self {
            Self::PrivateKeyJwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_private_key_jwt(self) -> Option<CreatePrivateKeyJwtRequest> {
        match self {
            Self::PrivateKeyJwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mtls(&self) -> bool {
        matches!(self, Self::Mtls { .. })
    }

    pub fn as_mtls(&self) -> Option<&CreateMtlsAuthRequest> {
        match self {
            Self::Mtls { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mtls(self) -> Option<CreateMtlsAuthRequest> {
        match self {
            Self::Mtls { data } => Some(data),
            _ => None,
        }
    }
}
