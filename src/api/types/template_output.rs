pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TemplateOutput {
    #[serde(rename = "array")]
    Array {
        #[serde(flatten)]
        data: TemplateArrayOutput,
    },

    #[serde(rename = "audio")]
    Audio {
        #[serde(flatten)]
        data: TemplateAudioOutput,
    },

    #[serde(rename = "boolean")]
    Boolean {
        #[serde(flatten)]
        data: TemplateBooleanOutput,
    },

    #[serde(rename = "image")]
    Image {
        #[serde(flatten)]
        data: TemplateImageOutput,
    },

    #[serde(rename = "integer")]
    Integer {
        #[serde(flatten)]
        data: TemplateIntegerOutput,
    },

    #[serde(rename = "number")]
    Number {
        #[serde(flatten)]
        data: TemplateNumberOutput,
    },

    #[serde(rename = "object")]
    Object {
        #[serde(flatten)]
        data: TemplateObjectOutput,
    },

    #[serde(rename = "string")]
    String {
        #[serde(flatten)]
        data: TemplateStringOutput,
    },

    #[serde(rename = "video")]
    Video {
        #[serde(flatten)]
        data: TemplateVideoOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TemplateOutput {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array { .. })
    }

    pub fn as_array(&self) -> Option<&TemplateArrayOutput> {
        match self {
            Self::Array { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<TemplateArrayOutput> {
        match self {
            Self::Array { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. })
    }

    pub fn as_audio(&self) -> Option<&TemplateAudioOutput> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio(self) -> Option<TemplateAudioOutput> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean { .. })
    }

    pub fn as_boolean(&self) -> Option<&TemplateBooleanOutput> {
        match self {
            Self::Boolean { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<TemplateBooleanOutput> {
        match self {
            Self::Boolean { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn as_image(&self) -> Option<&TemplateImageOutput> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_image(self) -> Option<TemplateImageOutput> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer { .. })
    }

    pub fn as_integer(&self) -> Option<&TemplateIntegerOutput> {
        match self {
            Self::Integer { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<TemplateIntegerOutput> {
        match self {
            Self::Integer { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number { .. })
    }

    pub fn as_number(&self) -> Option<&TemplateNumberOutput> {
        match self {
            Self::Number { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_number(self) -> Option<TemplateNumberOutput> {
        match self {
            Self::Number { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object { .. })
    }

    pub fn as_object(&self) -> Option<&TemplateObjectOutput> {
        match self {
            Self::Object { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<TemplateObjectOutput> {
        match self {
            Self::Object { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    pub fn as_string(&self) -> Option<&TemplateStringOutput> {
        match self {
            Self::String { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<TemplateStringOutput> {
        match self {
            Self::String { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video { .. })
    }

    pub fn as_video(&self) -> Option<&TemplateVideoOutput> {
        match self {
            Self::Video { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_video(self) -> Option<TemplateVideoOutput> {
        match self {
            Self::Video { data } => Some(data),
            _ => None,
        }
    }
}
