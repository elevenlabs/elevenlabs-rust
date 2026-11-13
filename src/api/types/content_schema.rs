pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContentSchema {
    #[serde(rename = "array")]
    Array {
        #[serde(flatten)]
        data: ArraySchema,
    },

    #[serde(rename = "audio")]
    Audio {
        #[serde(flatten)]
        data: AudioSchema,
    },

    #[serde(rename = "boolean")]
    Boolean {
        #[serde(flatten)]
        data: BooleanSchema,
    },

    #[serde(rename = "image")]
    Image {
        #[serde(flatten)]
        data: ImageSchema,
    },

    #[serde(rename = "integer")]
    Integer {
        #[serde(flatten)]
        data: IntegerSchema,
    },

    #[serde(rename = "number")]
    Number {
        #[serde(flatten)]
        data: NumberSchema,
    },

    #[serde(rename = "object")]
    Object {
        #[serde(flatten)]
        data: ObjectSchema,
    },

    #[serde(rename = "string")]
    String {
        #[serde(flatten)]
        data: StringSchema,
    },

    #[serde(rename = "video")]
    Video {
        #[serde(flatten)]
        data: VideoSchema,
    },

    #[serde(rename = "voice")]
    Voice {
        #[serde(flatten)]
        data: VoiceSchema,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ContentSchema {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array { .. })
    }

    pub fn as_array(&self) -> Option<&ArraySchema> {
        match self {
            Self::Array { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<ArraySchema> {
        match self {
            Self::Array { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. })
    }

    pub fn as_audio(&self) -> Option<&AudioSchema> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio(self) -> Option<AudioSchema> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean { .. })
    }

    pub fn as_boolean(&self) -> Option<&BooleanSchema> {
        match self {
            Self::Boolean { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<BooleanSchema> {
        match self {
            Self::Boolean { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn as_image(&self) -> Option<&ImageSchema> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_image(self) -> Option<ImageSchema> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer { .. })
    }

    pub fn as_integer(&self) -> Option<&IntegerSchema> {
        match self {
            Self::Integer { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<IntegerSchema> {
        match self {
            Self::Integer { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number { .. })
    }

    pub fn as_number(&self) -> Option<&NumberSchema> {
        match self {
            Self::Number { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_number(self) -> Option<NumberSchema> {
        match self {
            Self::Number { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object { .. })
    }

    pub fn as_object(&self) -> Option<&ObjectSchema> {
        match self {
            Self::Object { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<ObjectSchema> {
        match self {
            Self::Object { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    pub fn as_string(&self) -> Option<&StringSchema> {
        match self {
            Self::String { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<StringSchema> {
        match self {
            Self::String { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video { .. })
    }

    pub fn as_video(&self) -> Option<&VideoSchema> {
        match self {
            Self::Video { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_video(self) -> Option<VideoSchema> {
        match self {
            Self::Video { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_voice(&self) -> bool {
        matches!(self, Self::Voice { .. })
    }

    pub fn as_voice(&self) -> Option<&VoiceSchema> {
        match self {
            Self::Voice { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_voice(self) -> Option<VoiceSchema> {
        match self {
            Self::Voice { data } => Some(data),
            _ => None,
        }
    }
}
