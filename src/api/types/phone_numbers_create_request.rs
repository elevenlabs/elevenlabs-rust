pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "provider")]
#[non_exhaustive]
pub enum PhoneNumbersCreateRequest {
    #[serde(rename = "twilio")]
    Twilio {
        #[serde(flatten)]
        data: CreateTwilioPhoneNumberRequest,
    },

    #[serde(rename = "exotel")]
    Exotel {
        #[serde(flatten)]
        data: CreateExotelPhoneNumberRequest,
    },

    #[serde(rename = "sip_trunk")]
    SipTrunk {
        #[serde(flatten)]
        data: CreateSipTrunkPhoneNumberRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PhoneNumbersCreateRequest {
    pub fn is_twilio(&self) -> bool {
        matches!(self, Self::Twilio { .. })
    }

    pub fn as_twilio(&self) -> Option<&CreateTwilioPhoneNumberRequest> {
        match self {
            Self::Twilio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_twilio(self) -> Option<CreateTwilioPhoneNumberRequest> {
        match self {
            Self::Twilio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_exotel(&self) -> bool {
        matches!(self, Self::Exotel { .. })
    }

    pub fn as_exotel(&self) -> Option<&CreateExotelPhoneNumberRequest> {
        match self {
            Self::Exotel { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_exotel(self) -> Option<CreateExotelPhoneNumberRequest> {
        match self {
            Self::Exotel { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sip_trunk(&self) -> bool {
        matches!(self, Self::SipTrunk { .. })
    }

    pub fn as_sip_trunk(&self) -> Option<&CreateSipTrunkPhoneNumberRequest> {
        match self {
            Self::SipTrunk { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sip_trunk(self) -> Option<CreateSipTrunkPhoneNumberRequest> {
        match self {
            Self::SipTrunk { data } => Some(data),
            _ => None,
        }
    }
}
