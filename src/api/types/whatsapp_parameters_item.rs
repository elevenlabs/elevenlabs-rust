pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ParametersItem {
    WhatsAppTemplateTextParam(WhatsAppTemplateTextParam),
    WhatsAppTemplateImageParam(WhatsAppTemplateImageParam),
    WhatsAppTemplateDocumentParam(WhatsAppTemplateDocumentParam),
    WhatsAppTemplateLocationParam(WhatsAppTemplateLocationParam),

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    __Unknown(serde_json::Value),
}

impl<'de> serde::Deserialize<'de> for ParametersItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("text") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<WhatsAppTemplateTextParam>(value.clone()) {
                return Ok(Self::WhatsAppTemplateTextParam(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("image") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<WhatsAppTemplateImageParam>(value.clone())
            {
                return Ok(Self::WhatsAppTemplateImageParam(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("document") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<WhatsAppTemplateDocumentParam>(value.clone())
            {
                return Ok(Self::WhatsAppTemplateDocumentParam(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("location") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<WhatsAppTemplateLocationParam>(value.clone())
            {
                return Ok(Self::WhatsAppTemplateLocationParam(parsed));
            }
        }

        if !matched_literal {
            if value.get("type").is_some_and(serde_json::Value::is_string) {
                return Ok(Self::__Unknown(value));
            }

            if let Ok(parsed) = serde_json::from_value::<WhatsAppTemplateTextParam>(value.clone()) {
                return Ok(Self::WhatsAppTemplateTextParam(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<WhatsAppTemplateImageParam>(value.clone())
            {
                return Ok(Self::WhatsAppTemplateImageParam(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<WhatsAppTemplateDocumentParam>(value.clone())
            {
                return Ok(Self::WhatsAppTemplateDocumentParam(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<WhatsAppTemplateLocationParam>(value.clone())
            {
                return Ok(Self::WhatsAppTemplateLocationParam(parsed));
            }

            return Ok(Self::__Unknown(value));
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ParametersItem",
        ))
    }
}

impl ParametersItem {
    pub fn is_whats_app_template_text_param(&self) -> bool {
        matches!(self, Self::WhatsAppTemplateTextParam(_))
    }

    pub fn as_whats_app_template_text_param(&self) -> Option<&WhatsAppTemplateTextParam> {
        match self {
            Self::WhatsAppTemplateTextParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_whats_app_template_text_param(self) -> Option<WhatsAppTemplateTextParam> {
        match self {
            Self::WhatsAppTemplateTextParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_whats_app_template_image_param(&self) -> bool {
        matches!(self, Self::WhatsAppTemplateImageParam(_))
    }

    pub fn as_whats_app_template_image_param(&self) -> Option<&WhatsAppTemplateImageParam> {
        match self {
            Self::WhatsAppTemplateImageParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_whats_app_template_image_param(self) -> Option<WhatsAppTemplateImageParam> {
        match self {
            Self::WhatsAppTemplateImageParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_whats_app_template_document_param(&self) -> bool {
        matches!(self, Self::WhatsAppTemplateDocumentParam(_))
    }

    pub fn as_whats_app_template_document_param(&self) -> Option<&WhatsAppTemplateDocumentParam> {
        match self {
            Self::WhatsAppTemplateDocumentParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_whats_app_template_document_param(self) -> Option<WhatsAppTemplateDocumentParam> {
        match self {
            Self::WhatsAppTemplateDocumentParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_whats_app_template_location_param(&self) -> bool {
        matches!(self, Self::WhatsAppTemplateLocationParam(_))
    }

    pub fn as_whats_app_template_location_param(&self) -> Option<&WhatsAppTemplateLocationParam> {
        match self {
            Self::WhatsAppTemplateLocationParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_whats_app_template_location_param(self) -> Option<WhatsAppTemplateLocationParam> {
        match self {
            Self::WhatsAppTemplateLocationParam(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::__Unknown(_))
    }

    pub fn as_unknown(&self) -> Option<&serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_unknown(self) -> Option<serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ParametersItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WhatsAppTemplateTextParam(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WhatsAppTemplateImageParam(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WhatsAppTemplateDocumentParam(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WhatsAppTemplateLocationParam(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::__Unknown(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
