pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum LanguagesResponse {
    #[serde(rename = "pair")]
    Pair {
        #[serde(flatten)]
        data: PairedLanguagesResponse,
    },

    #[serde(rename = "single")]
    Single {
        #[serde(flatten)]
        data: SingleLanguagesResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl LanguagesResponse {
    pub fn is_pair(&self) -> bool {
        matches!(self, Self::Pair { .. })
    }

    pub fn as_pair(&self) -> Option<&PairedLanguagesResponse> {
        match self {
            Self::Pair { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_pair(self) -> Option<PairedLanguagesResponse> {
        match self {
            Self::Pair { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single { .. })
    }

    pub fn as_single(&self) -> Option<&SingleLanguagesResponse> {
        match self {
            Self::Single { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_single(self) -> Option<SingleLanguagesResponse> {
        match self {
            Self::Single { data } => Some(data),
            _ => None,
        }
    }
}
