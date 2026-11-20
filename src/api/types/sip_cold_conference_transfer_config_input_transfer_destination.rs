pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SipColdConferenceTransferConfigInputTransferDestination {
    #[serde(rename = "phone")]
    Phone {
        #[serde(flatten)]
        data: PhoneDestinationInput,
    },

    #[serde(rename = "phone_dynamic_variable")]
    PhoneDynamicVariable {
        #[serde(flatten)]
        data: PhoneDynamicVariableDestinationInput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SipColdConferenceTransferConfigInputTransferDestination {
    pub fn is_phone(&self) -> bool {
        matches!(self, Self::Phone { .. })
    }

    pub fn as_phone(&self) -> Option<&PhoneDestinationInput> {
        match self {
            Self::Phone { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone(self) -> Option<PhoneDestinationInput> {
        match self {
            Self::Phone { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_phone_dynamic_variable(&self) -> bool {
        matches!(self, Self::PhoneDynamicVariable { .. })
    }

    pub fn as_phone_dynamic_variable(&self) -> Option<&PhoneDynamicVariableDestinationInput> {
        match self {
            Self::PhoneDynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone_dynamic_variable(self) -> Option<PhoneDynamicVariableDestinationInput> {
        match self {
            Self::PhoneDynamicVariable { data } => Some(data),
            _ => None,
        }
    }
}
