pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TwilioTransferOutputConfig {
    #[serde(rename = "twilio_cold_blind_transfer")]
    TwilioColdBlindTransfer {
        #[serde(flatten)]
        data: TwilioColdBlindTransferConfigOutput,
    },

    #[serde(rename = "twilio_warm_conference_transfer")]
    TwilioWarmConferenceTransfer {
        #[serde(flatten)]
        data: TwilioWarmConferenceTransferConfigOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TwilioTransferOutputConfig {
    pub fn is_twilio_cold_blind_transfer(&self) -> bool {
        matches!(self, Self::TwilioColdBlindTransfer { .. })
    }

    pub fn as_twilio_cold_blind_transfer(&self) -> Option<&TwilioColdBlindTransferConfigOutput> {
        match self {
            Self::TwilioColdBlindTransfer { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_twilio_cold_blind_transfer(self) -> Option<TwilioColdBlindTransferConfigOutput> {
        match self {
            Self::TwilioColdBlindTransfer { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_twilio_warm_conference_transfer(&self) -> bool {
        matches!(self, Self::TwilioWarmConferenceTransfer { .. })
    }

    pub fn as_twilio_warm_conference_transfer(
        &self,
    ) -> Option<&TwilioWarmConferenceTransferConfigOutput> {
        match self {
            Self::TwilioWarmConferenceTransfer { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_twilio_warm_conference_transfer(
        self,
    ) -> Option<TwilioWarmConferenceTransferConfigOutput> {
        match self {
            Self::TwilioWarmConferenceTransfer { data } => Some(data),
            _ => None,
        }
    }
}
