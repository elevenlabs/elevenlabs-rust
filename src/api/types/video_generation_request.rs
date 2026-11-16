pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_id")]
#[non_exhaustive]
pub enum VideoGenerationRequest {
    #[serde(rename = "bytedance-seedance-v2")]
    BytedanceSeedanceV2 {
        #[serde(flatten)]
        data: BytedanceSeedance2Request,
    },

    #[serde(rename = "bytedance-seedance-v2-fast")]
    BytedanceSeedanceV2Fast {
        #[serde(flatten)]
        data: BytedanceSeedance2FastRequest,
    },

    #[serde(rename = "bytedance-seedance-v2-mini")]
    BytedanceSeedanceV2Mini {
        #[serde(flatten)]
        data: BytedanceSeedance2MiniRequest,
    },

    #[serde(rename = "bytedance-seedance-v2.5")]
    BytedanceSeedanceV2_5 {
        #[serde(flatten)]
        data: BytedanceSeedance25Request,
    },

    #[serde(rename = "creatify-aurora")]
    CreatifyAurora {
        #[serde(flatten)]
        data: CreatifyAuroraRequest,
    },

    #[serde(rename = "veo-3.1-fast-generate-001")]
    Veo3_1FastGenerate001 {
        #[serde(flatten)]
        data: Veo3_1FastRequest,
    },

    #[serde(rename = "veo-3.1-generate-001")]
    Veo3_1Generate001 {
        #[serde(flatten)]
        data: Veo3_1Request,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl VideoGenerationRequest {
    pub fn is_bytedance_seedance_v2(&self) -> bool {
        matches!(self, Self::BytedanceSeedanceV2 { .. })
    }

    pub fn as_bytedance_seedance_v2(&self) -> Option<&BytedanceSeedance2Request> {
        match self {
            Self::BytedanceSeedanceV2 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bytedance_seedance_v2(self) -> Option<BytedanceSeedance2Request> {
        match self {
            Self::BytedanceSeedanceV2 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bytedance_seedance_v2_fast(&self) -> bool {
        matches!(self, Self::BytedanceSeedanceV2Fast { .. })
    }

    pub fn as_bytedance_seedance_v2_fast(&self) -> Option<&BytedanceSeedance2FastRequest> {
        match self {
            Self::BytedanceSeedanceV2Fast { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bytedance_seedance_v2_fast(self) -> Option<BytedanceSeedance2FastRequest> {
        match self {
            Self::BytedanceSeedanceV2Fast { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bytedance_seedance_v2_mini(&self) -> bool {
        matches!(self, Self::BytedanceSeedanceV2Mini { .. })
    }

    pub fn as_bytedance_seedance_v2_mini(&self) -> Option<&BytedanceSeedance2MiniRequest> {
        match self {
            Self::BytedanceSeedanceV2Mini { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bytedance_seedance_v2_mini(self) -> Option<BytedanceSeedance2MiniRequest> {
        match self {
            Self::BytedanceSeedanceV2Mini { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bytedance_seedance_v2_5(&self) -> bool {
        matches!(self, Self::BytedanceSeedanceV2_5 { .. })
    }

    pub fn as_bytedance_seedance_v2_5(&self) -> Option<&BytedanceSeedance25Request> {
        match self {
            Self::BytedanceSeedanceV2_5 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bytedance_seedance_v2_5(self) -> Option<BytedanceSeedance25Request> {
        match self {
            Self::BytedanceSeedanceV2_5 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_creatify_aurora(&self) -> bool {
        matches!(self, Self::CreatifyAurora { .. })
    }

    pub fn as_creatify_aurora(&self) -> Option<&CreatifyAuroraRequest> {
        match self {
            Self::CreatifyAurora { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_creatify_aurora(self) -> Option<CreatifyAuroraRequest> {
        match self {
            Self::CreatifyAurora { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_veo3_1_fast_generate001(&self) -> bool {
        matches!(self, Self::Veo3_1FastGenerate001 { .. })
    }

    pub fn as_veo3_1_fast_generate001(&self) -> Option<&Veo3_1FastRequest> {
        match self {
            Self::Veo3_1FastGenerate001 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_veo3_1_fast_generate001(self) -> Option<Veo3_1FastRequest> {
        match self {
            Self::Veo3_1FastGenerate001 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_veo3_1_generate001(&self) -> bool {
        matches!(self, Self::Veo3_1Generate001 { .. })
    }

    pub fn as_veo3_1_generate001(&self) -> Option<&Veo3_1Request> {
        match self {
            Self::Veo3_1Generate001 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_veo3_1_generate001(self) -> Option<Veo3_1Request> {
        match self {
            Self::Veo3_1Generate001 { data } => Some(data),
            _ => None,
        }
    }
}
