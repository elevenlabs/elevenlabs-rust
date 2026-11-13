pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum DocumentsItem {
    #[serde(rename = "file")]
    File {
        #[serde(flatten)]
        data: GetKnowledgeBaseSummaryFileResponseModel,
    },

    #[serde(rename = "folder")]
    Folder {
        #[serde(flatten)]
        data: GetKnowledgeBaseSummaryFolderResponseModel,
    },

    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        data: GetKnowledgeBaseSummaryTextResponseModel,
    },

    #[serde(rename = "url")]
    Url {
        #[serde(flatten)]
        data: GetKnowledgeBaseSummaryUrlResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl DocumentsItem {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    pub fn as_file(&self) -> Option<&GetKnowledgeBaseSummaryFileResponseModel> {
        match self {
            Self::File { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_file(self) -> Option<GetKnowledgeBaseSummaryFileResponseModel> {
        match self {
            Self::File { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_folder(&self) -> bool {
        matches!(self, Self::Folder { .. })
    }

    pub fn as_folder(&self) -> Option<&GetKnowledgeBaseSummaryFolderResponseModel> {
        match self {
            Self::Folder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_folder(self) -> Option<GetKnowledgeBaseSummaryFolderResponseModel> {
        match self {
            Self::Folder { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn as_text(&self) -> Option<&GetKnowledgeBaseSummaryTextResponseModel> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<GetKnowledgeBaseSummaryTextResponseModel> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    pub fn as_url(&self) -> Option<&GetKnowledgeBaseSummaryUrlResponseModel> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_url(self) -> Option<GetKnowledgeBaseSummaryUrlResponseModel> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }
}
