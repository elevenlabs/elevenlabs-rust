pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateEnvironmentVariableRequestValues {
    String(String),
    EnvironmentVariableSecretValueRequest(EnvironmentVariableSecretValueRequest),
    EnvironmentVariableAuthConnectionValueRequest(EnvironmentVariableAuthConnectionValueRequest),
}

impl UpdateEnvironmentVariableRequestValues {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_environment_variable_secret_value_request(&self) -> bool {
        matches!(self, Self::EnvironmentVariableSecretValueRequest(_))
    }

    pub fn as_environment_variable_secret_value_request(
        &self,
    ) -> Option<&EnvironmentVariableSecretValueRequest> {
        match self {
            Self::EnvironmentVariableSecretValueRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_environment_variable_secret_value_request(
        self,
    ) -> Option<EnvironmentVariableSecretValueRequest> {
        match self {
            Self::EnvironmentVariableSecretValueRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_environment_variable_auth_connection_value_request(&self) -> bool {
        matches!(self, Self::EnvironmentVariableAuthConnectionValueRequest(_))
    }

    pub fn as_environment_variable_auth_connection_value_request(
        &self,
    ) -> Option<&EnvironmentVariableAuthConnectionValueRequest> {
        match self {
            Self::EnvironmentVariableAuthConnectionValueRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_environment_variable_auth_connection_value_request(
        self,
    ) -> Option<EnvironmentVariableAuthConnectionValueRequest> {
        match self {
            Self::EnvironmentVariableAuthConnectionValueRequest(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdateEnvironmentVariableRequestValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::EnvironmentVariableSecretValueRequest(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::EnvironmentVariableAuthConnectionValueRequest(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for UpdateEnvironmentVariableRequestValues {
    fn default() -> Self {
        UpdateEnvironmentVariableRequestValues::String(Default::default())
    }
}
