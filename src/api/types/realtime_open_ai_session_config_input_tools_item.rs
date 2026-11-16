pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OpenAiSessionConfigInputToolsItem {
    #[serde(rename = "function")]
    Function {
        #[serde(flatten)]
        data: OpenAiFunctionTool,
    },

    #[serde(rename = "mcp")]
    Mcp {
        #[serde(flatten)]
        data: OpenAimcpTool,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OpenAiSessionConfigInputToolsItem {
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function { .. })
    }

    pub fn as_function(&self) -> Option<&OpenAiFunctionTool> {
        match self {
            Self::Function { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_function(self) -> Option<OpenAiFunctionTool> {
        match self {
            Self::Function { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mcp(&self) -> bool {
        matches!(self, Self::Mcp { .. })
    }

    pub fn as_mcp(&self) -> Option<&OpenAimcpTool> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mcp(self) -> Option<OpenAimcpTool> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }
}
