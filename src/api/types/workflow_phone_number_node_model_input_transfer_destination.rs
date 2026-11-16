pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowPhoneNumberNodeModelInputTransferDestination {
    #[serde(rename = "phone")]
    Phone {
        #[serde(flatten)]
        data: PhoneNumberTransferDestination,
    },

    #[serde(rename = "phone_dynamic_variable")]
    PhoneDynamicVariable {
        #[serde(flatten)]
        data: PhoneNumberDynamicVariableTransferDestination,
    },

    #[serde(rename = "sip_uri")]
    SipUri {
        #[serde(flatten)]
        data: SipUriTransferDestination,
    },

    #[serde(rename = "sip_uri_dynamic_variable")]
    SipUriDynamicVariable {
        #[serde(flatten)]
        data: SipUriDynamicVariableTransferDestination,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WorkflowPhoneNumberNodeModelInputTransferDestination {
    pub fn is_phone(&self) -> bool {
        matches!(self, Self::Phone { .. })
    }

    pub fn as_phone(&self) -> Option<&PhoneNumberTransferDestination> {
        match self {
            Self::Phone { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone(self) -> Option<PhoneNumberTransferDestination> {
        match self {
            Self::Phone { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_phone_dynamic_variable(&self) -> bool {
        matches!(self, Self::PhoneDynamicVariable { .. })
    }

    pub fn as_phone_dynamic_variable(
        &self,
    ) -> Option<&PhoneNumberDynamicVariableTransferDestination> {
        match self {
            Self::PhoneDynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone_dynamic_variable(
        self,
    ) -> Option<PhoneNumberDynamicVariableTransferDestination> {
        match self {
            Self::PhoneDynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sip_uri(&self) -> bool {
        matches!(self, Self::SipUri { .. })
    }

    pub fn as_sip_uri(&self) -> Option<&SipUriTransferDestination> {
        match self {
            Self::SipUri { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sip_uri(self) -> Option<SipUriTransferDestination> {
        match self {
            Self::SipUri { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sip_uri_dynamic_variable(&self) -> bool {
        matches!(self, Self::SipUriDynamicVariable { .. })
    }

    pub fn as_sip_uri_dynamic_variable(&self) -> Option<&SipUriDynamicVariableTransferDestination> {
        match self {
            Self::SipUriDynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sip_uri_dynamic_variable(self) -> Option<SipUriDynamicVariableTransferDestination> {
        match self {
            Self::SipUriDynamicVariable { data } => Some(data),
            _ => None,
        }
    }
}
