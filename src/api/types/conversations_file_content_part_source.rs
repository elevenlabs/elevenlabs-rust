pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum FileContentPartSource {
    #[serde(rename = "base64")]
    Base64 {
        #[serde(flatten)]
        data: Base64FileSource,
    },

    #[serde(rename = "file_id")]
    FileId {
        #[serde(flatten)]
        data: FileIdSource,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl FileContentPartSource {
    pub fn is_base64(&self) -> bool {
        matches!(self, Self::Base64 { .. })
    }

    pub fn as_base64(&self) -> Option<&Base64FileSource> {
        match self {
            Self::Base64 { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_base64(self) -> Option<Base64FileSource> {
        match self {
            Self::Base64 { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_file_id(&self) -> bool {
        matches!(self, Self::FileId { .. })
    }

    pub fn as_file_id(&self) -> Option<&FileIdSource> {
        match self {
            Self::FileId { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_file_id(self) -> Option<FileIdSource> {
        match self {
            Self::FileId { data } => Some(data),
            _ => None,
        }
    }
}
