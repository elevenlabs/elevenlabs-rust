pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "auth_type")]
#[non_exhaustive]
pub enum AuthConnectionsUpdateRequest {
    #[serde(rename = "oauth2_client_credentials")]
    Oauth2ClientCredentials {
        #[serde(flatten)]
        data: UpdateOAuth2ClientCredsRequest,
    },

    #[serde(rename = "basic_auth")]
    BasicAuth {
        #[serde(flatten)]
        data: UpdateBasicAuthRequest,
    },

    #[serde(rename = "bearer_auth")]
    BearerAuth {
        #[serde(flatten)]
        data: UpdateBearerAuthRequest,
    },

    #[serde(rename = "oauth2_jwt")]
    Oauth2Jwt {
        #[serde(flatten)]
        data: UpdateOAuth2JwtRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AuthConnectionsUpdateRequest {
    pub fn is_oauth2_client_credentials(&self) -> bool {
        matches!(self, Self::Oauth2ClientCredentials { .. })
    }

    pub fn as_oauth2_client_credentials(&self) -> Option<&UpdateOAuth2ClientCredsRequest> {
        match self {
            Self::Oauth2ClientCredentials { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_oauth2_client_credentials(self) -> Option<UpdateOAuth2ClientCredsRequest> {
        match self {
            Self::Oauth2ClientCredentials { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_basic_auth(&self) -> bool {
        matches!(self, Self::BasicAuth { .. })
    }

    pub fn as_basic_auth(&self) -> Option<&UpdateBasicAuthRequest> {
        match self {
            Self::BasicAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_basic_auth(self) -> Option<UpdateBasicAuthRequest> {
        match self {
            Self::BasicAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bearer_auth(&self) -> bool {
        matches!(self, Self::BearerAuth { .. })
    }

    pub fn as_bearer_auth(&self) -> Option<&UpdateBearerAuthRequest> {
        match self {
            Self::BearerAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bearer_auth(self) -> Option<UpdateBearerAuthRequest> {
        match self {
            Self::BearerAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_oauth2_jwt(&self) -> bool {
        matches!(self, Self::Oauth2Jwt { .. })
    }

    pub fn as_oauth2_jwt(&self) -> Option<&UpdateOAuth2JwtRequest> {
        match self {
            Self::Oauth2Jwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_oauth2_jwt(self) -> Option<UpdateOAuth2JwtRequest> {
        match self {
            Self::Oauth2Jwt { data } => Some(data),
            _ => None,
        }
    }
}
