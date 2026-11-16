pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TemplateParamsItem {
    #[serde(rename = "body")]
    Body {
        #[serde(flatten)]
        data: WhatsAppTemplateBodyComponentParams,
    },

    #[serde(rename = "button")]
    Button {
        #[serde(flatten)]
        data: WhatsAppTemplateButtonComponentParams,
    },

    #[serde(rename = "header")]
    Header {
        #[serde(flatten)]
        data: WhatsAppTemplateHeaderComponentParams,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TemplateParamsItem {
    pub fn is_body(&self) -> bool {
        matches!(self, Self::Body { .. })
    }

    pub fn as_body(&self) -> Option<&WhatsAppTemplateBodyComponentParams> {
        match self {
            Self::Body { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_body(self) -> Option<WhatsAppTemplateBodyComponentParams> {
        match self {
            Self::Body { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_button(&self) -> bool {
        matches!(self, Self::Button { .. })
    }

    pub fn as_button(&self) -> Option<&WhatsAppTemplateButtonComponentParams> {
        match self {
            Self::Button { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_button(self) -> Option<WhatsAppTemplateButtonComponentParams> {
        match self {
            Self::Button { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_header(&self) -> bool {
        matches!(self, Self::Header { .. })
    }

    pub fn as_header(&self) -> Option<&WhatsAppTemplateHeaderComponentParams> {
        match self {
            Self::Header { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_header(self) -> Option<WhatsAppTemplateHeaderComponentParams> {
        match self {
            Self::Header { data } => Some(data),
            _ => None,
        }
    }
}
