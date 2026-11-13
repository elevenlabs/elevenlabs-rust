pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PhoneNumberTransferCustomSipHeadersItem {
    #[serde(rename = "dynamic")]
    Dynamic {
        #[serde(flatten)]
        data: CustomSipHeaderWithDynamicVariable,
    },

    #[serde(rename = "static")]
    Static {
        #[serde(flatten)]
        data: CustomSipHeader,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PhoneNumberTransferCustomSipHeadersItem {
    pub fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic { .. })
    }

    pub fn as_dynamic(&self) -> Option<&CustomSipHeaderWithDynamicVariable> {
        match self {
            Self::Dynamic { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_dynamic(self) -> Option<CustomSipHeaderWithDynamicVariable> {
        match self {
            Self::Dynamic { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static { .. })
    }

    pub fn as_static(&self) -> Option<&CustomSipHeader> {
        match self {
            Self::Static { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_static(self) -> Option<CustomSipHeader> {
        match self {
            Self::Static { data } => Some(data),
            _ => None,
        }
    }
}
