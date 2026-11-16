pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ParametersItem {
    #[serde(rename = "document")]
    Document {
        #[serde(flatten)]
        data: WhatsAppTemplateDocumentParam,
    },

    #[serde(rename = "image")]
    Image {
        #[serde(flatten)]
        data: WhatsAppTemplateImageParam,
    },

    #[serde(rename = "location")]
    Location {
        #[serde(flatten)]
        data: WhatsAppTemplateLocationParam,
    },

    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        data: WhatsAppTemplateTextParam,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ParametersItem {
    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document { .. })
    }

    pub fn as_document(&self) -> Option<&WhatsAppTemplateDocumentParam> {
        match self {
            Self::Document { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_document(self) -> Option<WhatsAppTemplateDocumentParam> {
        match self {
            Self::Document { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn as_image(&self) -> Option<&WhatsAppTemplateImageParam> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_image(self) -> Option<WhatsAppTemplateImageParam> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_location(&self) -> bool {
        matches!(self, Self::Location { .. })
    }

    pub fn as_location(&self) -> Option<&WhatsAppTemplateLocationParam> {
        match self {
            Self::Location { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_location(self) -> Option<WhatsAppTemplateLocationParam> {
        match self {
            Self::Location { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn as_text(&self) -> Option<&WhatsAppTemplateTextParam> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<WhatsAppTemplateTextParam> {
        match self {
            Self::Text { data } => Some(data),
            _ => None,
        }
    }
}
