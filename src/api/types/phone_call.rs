pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PhoneCall {
    #[serde(rename = "exotel")]
    Exotel {
        #[serde(flatten)]
        data: ConversationHistoryExotelPhoneCallModel,
    },

    #[serde(rename = "sip_trunking")]
    SipTrunking {
        #[serde(flatten)]
        data: ConversationHistorySipTrunkingPhoneCallModel,
    },

    #[serde(rename = "twilio")]
    Twilio {
        #[serde(flatten)]
        data: ConversationHistoryTwilioPhoneCallModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PhoneCall {
    pub fn is_exotel(&self) -> bool {
        matches!(self, Self::Exotel { .. })
    }

    pub fn as_exotel(&self) -> Option<&ConversationHistoryExotelPhoneCallModel> {
        match self {
            Self::Exotel { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_exotel(self) -> Option<ConversationHistoryExotelPhoneCallModel> {
        match self {
            Self::Exotel { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sip_trunking(&self) -> bool {
        matches!(self, Self::SipTrunking { .. })
    }

    pub fn as_sip_trunking(&self) -> Option<&ConversationHistorySipTrunkingPhoneCallModel> {
        match self {
            Self::SipTrunking { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sip_trunking(self) -> Option<ConversationHistorySipTrunkingPhoneCallModel> {
        match self {
            Self::SipTrunking { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_twilio(&self) -> bool {
        matches!(self, Self::Twilio { .. })
    }

    pub fn as_twilio(&self) -> Option<&ConversationHistoryTwilioPhoneCallModel> {
        match self {
            Self::Twilio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_twilio(self) -> Option<ConversationHistoryTwilioPhoneCallModel> {
        match self {
            Self::Twilio { data } => Some(data),
            _ => None,
        }
    }
}
