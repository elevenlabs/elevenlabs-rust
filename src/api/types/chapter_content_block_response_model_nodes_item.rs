pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChapterContentBlockResponseModelNodesItem {
    #[serde(rename = "tts_node")]
    TtsNode {
        #[serde(flatten)]
        data: ChapterContentBlockTtsNodeResponseModel,
    },

    #[serde(rename = "_other")]
    _Other {
        #[serde(flatten)]
        data: ChapterContentBlockExtendableNodeResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ChapterContentBlockResponseModelNodesItem {
    pub fn is_tts_node(&self) -> bool {
        matches!(self, Self::TtsNode { .. })
    }

    pub fn as_tts_node(&self) -> Option<&ChapterContentBlockTtsNodeResponseModel> {
        match self {
            Self::TtsNode { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_tts_node(self) -> Option<ChapterContentBlockTtsNodeResponseModel> {
        match self {
            Self::TtsNode { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_other(&self) -> bool {
        matches!(self, Self::_Other { .. })
    }

    pub fn as_other(&self) -> Option<&ChapterContentBlockExtendableNodeResponseModel> {
        match self {
            Self::_Other { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_other(self) -> Option<ChapterContentBlockExtendableNodeResponseModel> {
        match self {
            Self::_Other { data } => Some(data),
            _ => None,
        }
    }
}
