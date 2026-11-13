pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_id")]
#[non_exhaustive]
pub enum TextToSpeechGenerationRequest {
    #[serde(rename = "eleven_flash_v2_5")]
    ElevenFlashV2_5 {
        #[serde(flatten)]
        data: ElevenFlashV2_5Request,
    },

    #[serde(rename = "eleven_multilingual_v2")]
    ElevenMultilingualV2 {
        #[serde(flatten)]
        data: ElevenMultilingualV2Request,
    },

    #[serde(rename = "eleven_v3")]
    ElevenV3 {
        #[serde(flatten)]
        data: ElevenV3Request,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TextToSpeechGenerationRequest {
    pub fn is_eleven_flash_v2_5(&self) -> bool {
        matches!(self, Self::ElevenFlashV2_5 { .. })
    }

    pub fn as_eleven_flash_v2_5(&self) -> Option<&ElevenFlashV2_5Request> {
        match self {
            Self::ElevenFlashV2_5 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_eleven_flash_v2_5(self) -> Option<ElevenFlashV2_5Request> {
        match self {
            Self::ElevenFlashV2_5 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_eleven_multilingual_v2(&self) -> bool {
        matches!(self, Self::ElevenMultilingualV2 { .. })
    }

    pub fn as_eleven_multilingual_v2(&self) -> Option<&ElevenMultilingualV2Request> {
        match self {
            Self::ElevenMultilingualV2 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_eleven_multilingual_v2(self) -> Option<ElevenMultilingualV2Request> {
        match self {
            Self::ElevenMultilingualV2 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_eleven_v3(&self) -> bool {
        matches!(self, Self::ElevenV3 { .. })
    }

    pub fn as_eleven_v3(&self) -> Option<&ElevenV3Request> {
        match self {
            Self::ElevenV3 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_eleven_v3(self) -> Option<ElevenV3Request> {
        match self {
            Self::ElevenV3 { data } => Some(data),
            _ => None,
        }
    }
}
