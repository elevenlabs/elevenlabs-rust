pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "preference")]
#[non_exhaustive]
pub enum PromptAgentApiModelWorkflowOverrideOutputBackupLlmConfig {
    #[serde(rename = "default")]
    Default {
        #[serde(flatten)]
        data: BackupLlmDefault,
    },

    #[serde(rename = "disabled")]
    Disabled {
        #[serde(flatten)]
        data: BackupLlmDisabled,
    },

    #[serde(rename = "override")]
    Override {
        #[serde(flatten)]
        data: BackupLlmOverride,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PromptAgentApiModelWorkflowOverrideOutputBackupLlmConfig {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default { .. })
    }

    pub fn as_default(&self) -> Option<&BackupLlmDefault> {
        match self {
            Self::Default { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_default(self) -> Option<BackupLlmDefault> {
        match self {
            Self::Default { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled { .. })
    }

    pub fn as_disabled(&self) -> Option<&BackupLlmDisabled> {
        match self {
            Self::Disabled { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_disabled(self) -> Option<BackupLlmDisabled> {
        match self {
            Self::Disabled { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_override(&self) -> bool {
        matches!(self, Self::Override { .. })
    }

    pub fn as_override(&self) -> Option<&BackupLlmOverride> {
        match self {
            Self::Override { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_override(self) -> Option<BackupLlmOverride> {
        match self {
            Self::Override { data } => Some(data),
            _ => None,
        }
    }
}
