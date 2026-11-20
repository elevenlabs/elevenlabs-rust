pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContentItem {
    #[serde(rename = "file")]
    File {
        #[serde(flatten)]
        data: FileContentPart,
    },

    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        data: TextContentPart,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ContentItem {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    pub fn as_file(&self) -> Option<&FileContentPart> {
        match self {
            Self::File { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_file(self) -> Option<FileContentPart> {
        match self {
            Self::File { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn as_text(&self) -> Option<&TextContentPart> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<TextContentPart> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }
}
