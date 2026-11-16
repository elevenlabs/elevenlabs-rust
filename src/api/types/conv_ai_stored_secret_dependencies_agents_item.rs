pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConvAiStoredSecretDependenciesAgentsItem {
    #[serde(rename = "available")]
    Available {
        #[serde(flatten)]
        data: DependentAvailableAgentIdentifier,
    },

    #[serde(rename = "unknown")]
    Unknown {
        #[serde(flatten)]
        data: DependentUnknownAgentIdentifier,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConvAiStoredSecretDependenciesAgentsItem {
    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub fn as_available(&self) -> Option<&DependentAvailableAgentIdentifier> {
        match self {
            Self::Available { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_available(self) -> Option<DependentAvailableAgentIdentifier> {
        match self {
            Self::Available { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }

    pub fn as_unknown(&self) -> Option<&DependentUnknownAgentIdentifier> {
        match self {
            Self::Unknown { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_unknown(self) -> Option<DependentUnknownAgentIdentifier> {
        match self {
            Self::Unknown { data } => Some(data),
            _ => None,
        }
    }
}
