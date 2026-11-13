pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ImageReference {
    #[serde(rename = "asset")]
    Asset {
        #[serde(flatten)]
        data: StaticAssetReference,
    },

    #[serde(rename = "generation")]
    Generation {
        #[serde(flatten)]
        data: GenerationReference,
    },

    #[serde(rename = "inline_base64")]
    InlineBase64 {
        #[serde(flatten)]
        data: InlineImageReference,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ImageReference {
    pub fn is_asset(&self) -> bool {
        matches!(self, Self::Asset { .. })
    }

    pub fn as_asset(&self) -> Option<&StaticAssetReference> {
        match self {
            Self::Asset { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_asset(self) -> Option<StaticAssetReference> {
        match self {
            Self::Asset { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_generation(&self) -> bool {
        matches!(self, Self::Generation { .. })
    }

    pub fn as_generation(&self) -> Option<&GenerationReference> {
        match self {
            Self::Generation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_generation(self) -> Option<GenerationReference> {
        match self {
            Self::Generation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_inline_base64(&self) -> bool {
        matches!(self, Self::InlineBase64 { .. })
    }

    pub fn as_inline_base64(&self) -> Option<&InlineImageReference> {
        match self {
            Self::InlineBase64 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_inline_base64(self) -> Option<InlineImageReference> {
        match self {
            Self::InlineBase64 { data } => Some(data),
            _ => None,
        }
    }
}
