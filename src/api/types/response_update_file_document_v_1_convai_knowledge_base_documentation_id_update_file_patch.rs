pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponseUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
    #[serde(rename = "url")]
    Url {
        #[serde(flatten)]
        data: GetKnowledgeBaseUrlResponseModel,
    },

    #[serde(rename = "file")]
    File {
        #[serde(flatten)]
        data: GetKnowledgeBaseFileResponseModel,
    },

    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        data: GetKnowledgeBaseTextResponseModel,
    },

    #[serde(rename = "folder")]
    Folder {
        #[serde(flatten)]
        data: GetKnowledgeBaseFolderResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponseUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    pub fn as_url(&self) -> Option<&GetKnowledgeBaseUrlResponseModel> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_url(self) -> Option<GetKnowledgeBaseUrlResponseModel> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    pub fn as_file(&self) -> Option<&GetKnowledgeBaseFileResponseModel> {
        match self {
            Self::File { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_file(self) -> Option<GetKnowledgeBaseFileResponseModel> {
        match self {
            Self::File { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn as_text(&self) -> Option<&GetKnowledgeBaseTextResponseModel> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<GetKnowledgeBaseTextResponseModel> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_folder(&self) -> bool {
        matches!(self, Self::Folder { .. })
    }

    pub fn as_folder(&self) -> Option<&GetKnowledgeBaseFolderResponseModel> {
        match self {
            Self::Folder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_folder(self) -> Option<GetKnowledgeBaseFolderResponseModel> {
        match self {
            Self::Folder { data } => Some(data),
            _ => None,
        }
    }
}
