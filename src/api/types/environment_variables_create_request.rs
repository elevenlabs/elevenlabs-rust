pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum EnvironmentVariablesCreateRequest {
    #[serde(rename = "string")]
    String {
        #[serde(flatten)]
        data: CreateStringEnvironmentVariableRequest,
    },

    #[serde(rename = "secret")]
    Secret {
        #[serde(flatten)]
        data: CreateSecretEnvironmentVariableRequest,
    },

    #[serde(rename = "auth_connection")]
    AuthConnection {
        #[serde(flatten)]
        data: CreateAuthConnectionEnvironmentVariableRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl EnvironmentVariablesCreateRequest {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    pub fn as_string(&self) -> Option<&CreateStringEnvironmentVariableRequest> {
        match self {
            Self::String { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<CreateStringEnvironmentVariableRequest> {
        match self {
            Self::String { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_secret(&self) -> bool {
        matches!(self, Self::Secret { .. })
    }

    pub fn as_secret(&self) -> Option<&CreateSecretEnvironmentVariableRequest> {
        match self {
            Self::Secret { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_secret(self) -> Option<CreateSecretEnvironmentVariableRequest> {
        match self {
            Self::Secret { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_auth_connection(&self) -> bool {
        matches!(self, Self::AuthConnection { .. })
    }

    pub fn as_auth_connection(&self) -> Option<&CreateAuthConnectionEnvironmentVariableRequest> {
        match self {
            Self::AuthConnection { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_auth_connection(self) -> Option<CreateAuthConnectionEnvironmentVariableRequest> {
        match self {
            Self::AuthConnection { data } => Some(data),
            _ => None,
        }
    }
}
