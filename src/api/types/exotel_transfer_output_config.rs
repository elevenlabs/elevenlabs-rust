pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ExotelTransferOutputConfig {
    #[serde(rename = "exotel_cold_blind_transfer")]
    ExotelColdBlindTransfer {
        #[serde(flatten)]
        data: ExotelColdBlindTransferConfigOutput,
    },

    #[serde(rename = "exotel_warm_transfer")]
    ExotelWarmTransfer {
        #[serde(flatten)]
        data: ExotelWarmTransferConfigOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ExotelTransferOutputConfig {
    pub fn is_exotel_cold_blind_transfer(&self) -> bool {
        matches!(self, Self::ExotelColdBlindTransfer { .. })
    }

    pub fn as_exotel_cold_blind_transfer(&self) -> Option<&ExotelColdBlindTransferConfigOutput> {
        match self {
            Self::ExotelColdBlindTransfer { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_exotel_cold_blind_transfer(self) -> Option<ExotelColdBlindTransferConfigOutput> {
        match self {
            Self::ExotelColdBlindTransfer { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_exotel_warm_transfer(&self) -> bool {
        matches!(self, Self::ExotelWarmTransfer { .. })
    }

    pub fn as_exotel_warm_transfer(&self) -> Option<&ExotelWarmTransferConfigOutput> {
        match self {
            Self::ExotelWarmTransfer { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_exotel_warm_transfer(self) -> Option<ExotelWarmTransferConfigOutput> {
        match self {
            Self::ExotelWarmTransfer { data } => Some(data),
            _ => None,
        }
    }
}
