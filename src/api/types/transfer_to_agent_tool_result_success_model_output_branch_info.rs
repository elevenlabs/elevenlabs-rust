pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "branch_reason")]
#[non_exhaustive]
pub enum TransferToAgentToolResultSuccessModelOutputBranchInfo {
    #[serde(rename = "configured")]
    Configured {
        #[serde(flatten)]
        data: TransferBranchInfoConfigured,
    },

    #[serde(rename = "defaulting_to_main")]
    DefaultingToMain {
        #[serde(flatten)]
        data: TransferBranchInfoDefaultingToMain,
    },

    #[serde(rename = "traffic_split")]
    TrafficSplit {
        #[serde(flatten)]
        data: TransferBranchInfoTrafficSplit,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TransferToAgentToolResultSuccessModelOutputBranchInfo {
    pub fn is_configured(&self) -> bool {
        matches!(self, Self::Configured { .. })
    }

    pub fn as_configured(&self) -> Option<&TransferBranchInfoConfigured> {
        match self {
            Self::Configured { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_configured(self) -> Option<TransferBranchInfoConfigured> {
        match self {
            Self::Configured { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_defaulting_to_main(&self) -> bool {
        matches!(self, Self::DefaultingToMain { .. })
    }

    pub fn as_defaulting_to_main(&self) -> Option<&TransferBranchInfoDefaultingToMain> {
        match self {
            Self::DefaultingToMain { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_defaulting_to_main(self) -> Option<TransferBranchInfoDefaultingToMain> {
        match self {
            Self::DefaultingToMain { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_traffic_split(&self) -> bool {
        matches!(self, Self::TrafficSplit { .. })
    }

    pub fn as_traffic_split(&self) -> Option<&TransferBranchInfoTrafficSplit> {
        match self {
            Self::TrafficSplit { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_traffic_split(self) -> Option<TransferBranchInfoTrafficSplit> {
        match self {
            Self::TrafficSplit { data } => Some(data),
            _ => None,
        }
    }
}
