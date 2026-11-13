pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_id")]
#[non_exhaustive]
pub enum ImageGenerationRequest {
    #[serde(rename = "bytedance-seedream-5-lite")]
    BytedanceSeedream5Lite {
        #[serde(flatten)]
        data: BytedanceSeedream5LiteRequest,
    },

    #[serde(rename = "bytedance-seedream-5-pro")]
    BytedanceSeedream5Pro {
        #[serde(flatten)]
        data: BytedanceSeedream5ProRequest,
    },

    #[serde(rename = "gemini-2.5-flash-image")]
    Gemini2_5FlashImage {
        #[serde(flatten)]
        data: Gemini25FlashImageRequest,
    },

    #[serde(rename = "gemini-3-pro-image")]
    Gemini3ProImage {
        #[serde(flatten)]
        data: Gemini3ProImageRequest,
    },

    #[serde(rename = "gemini-3.1-flash-image")]
    Gemini3_1FlashImage {
        #[serde(flatten)]
        data: Gemini31FlashImageRequest,
    },

    #[serde(rename = "gemini-3.1-flash-lite-image")]
    Gemini3_1FlashLiteImage {
        #[serde(flatten)]
        data: Gemini31FlashLiteImageRequest,
    },

    #[serde(rename = "gpt-image-1")]
    GptImage1 {
        #[serde(flatten)]
        data: GptImage1Request,
    },

    #[serde(rename = "gpt-image-1.5")]
    GptImage1_5 {
        #[serde(flatten)]
        data: GptImage1_5Request,
    },

    #[serde(rename = "gpt-image-2")]
    GptImage2 {
        #[serde(flatten)]
        data: GptImage2Request,
    },

    #[serde(rename = "gpt-image-2.5-flare")]
    GptImage2_5Flare {
        #[serde(flatten)]
        data: GptImage25FlareRequest,
    },

    #[serde(rename = "gpt-image-2.5-sunburst")]
    GptImage2_5Sunburst {
        #[serde(flatten)]
        data: GptImage25SunburstRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ImageGenerationRequest {
    pub fn is_bytedance_seedream5_lite(&self) -> bool {
        matches!(self, Self::BytedanceSeedream5Lite { .. })
    }

    pub fn as_bytedance_seedream5_lite(&self) -> Option<&BytedanceSeedream5LiteRequest> {
        match self {
            Self::BytedanceSeedream5Lite { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bytedance_seedream5_lite(self) -> Option<BytedanceSeedream5LiteRequest> {
        match self {
            Self::BytedanceSeedream5Lite { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bytedance_seedream5_pro(&self) -> bool {
        matches!(self, Self::BytedanceSeedream5Pro { .. })
    }

    pub fn as_bytedance_seedream5_pro(&self) -> Option<&BytedanceSeedream5ProRequest> {
        match self {
            Self::BytedanceSeedream5Pro { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bytedance_seedream5_pro(self) -> Option<BytedanceSeedream5ProRequest> {
        match self {
            Self::BytedanceSeedream5Pro { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gemini2_5_flash_image(&self) -> bool {
        matches!(self, Self::Gemini2_5FlashImage { .. })
    }

    pub fn as_gemini2_5_flash_image(&self) -> Option<&Gemini25FlashImageRequest> {
        match self {
            Self::Gemini2_5FlashImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gemini2_5_flash_image(self) -> Option<Gemini25FlashImageRequest> {
        match self {
            Self::Gemini2_5FlashImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gemini3_pro_image(&self) -> bool {
        matches!(self, Self::Gemini3ProImage { .. })
    }

    pub fn as_gemini3_pro_image(&self) -> Option<&Gemini3ProImageRequest> {
        match self {
            Self::Gemini3ProImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gemini3_pro_image(self) -> Option<Gemini3ProImageRequest> {
        match self {
            Self::Gemini3ProImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gemini3_1_flash_image(&self) -> bool {
        matches!(self, Self::Gemini3_1FlashImage { .. })
    }

    pub fn as_gemini3_1_flash_image(&self) -> Option<&Gemini31FlashImageRequest> {
        match self {
            Self::Gemini3_1FlashImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gemini3_1_flash_image(self) -> Option<Gemini31FlashImageRequest> {
        match self {
            Self::Gemini3_1FlashImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gemini3_1_flash_lite_image(&self) -> bool {
        matches!(self, Self::Gemini3_1FlashLiteImage { .. })
    }

    pub fn as_gemini3_1_flash_lite_image(&self) -> Option<&Gemini31FlashLiteImageRequest> {
        match self {
            Self::Gemini3_1FlashLiteImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gemini3_1_flash_lite_image(self) -> Option<Gemini31FlashLiteImageRequest> {
        match self {
            Self::Gemini3_1FlashLiteImage { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gpt_image1(&self) -> bool {
        matches!(self, Self::GptImage1 { .. })
    }

    pub fn as_gpt_image1(&self) -> Option<&GptImage1Request> {
        match self {
            Self::GptImage1 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gpt_image1(self) -> Option<GptImage1Request> {
        match self {
            Self::GptImage1 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gpt_image1_5(&self) -> bool {
        matches!(self, Self::GptImage1_5 { .. })
    }

    pub fn as_gpt_image1_5(&self) -> Option<&GptImage1_5Request> {
        match self {
            Self::GptImage1_5 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gpt_image1_5(self) -> Option<GptImage1_5Request> {
        match self {
            Self::GptImage1_5 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gpt_image2(&self) -> bool {
        matches!(self, Self::GptImage2 { .. })
    }

    pub fn as_gpt_image2(&self) -> Option<&GptImage2Request> {
        match self {
            Self::GptImage2 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gpt_image2(self) -> Option<GptImage2Request> {
        match self {
            Self::GptImage2 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gpt_image2_5_flare(&self) -> bool {
        matches!(self, Self::GptImage2_5Flare { .. })
    }

    pub fn as_gpt_image2_5_flare(&self) -> Option<&GptImage25FlareRequest> {
        match self {
            Self::GptImage2_5Flare { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gpt_image2_5_flare(self) -> Option<GptImage25FlareRequest> {
        match self {
            Self::GptImage2_5Flare { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gpt_image2_5_sunburst(&self) -> bool {
        matches!(self, Self::GptImage2_5Sunburst { .. })
    }

    pub fn as_gpt_image2_5_sunburst(&self) -> Option<&GptImage25SunburstRequest> {
        match self {
            Self::GptImage2_5Sunburst { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gpt_image2_5_sunburst(self) -> Option<GptImage25SunburstRequest> {
        match self {
            Self::GptImage2_5Sunburst { data } => Some(data),
            _ => None,
        }
    }
}
