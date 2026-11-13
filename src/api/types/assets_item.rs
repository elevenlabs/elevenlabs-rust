pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AssetsItem {
    #[serde(rename = "video")]
    Video {
        #[serde(flatten)]
        data: ProjectVideoResponseModel,
    },

    #[serde(rename = "audio")]
    Audio {
        #[serde(flatten)]
        data: ProjectExternalAudioResponseModel,
    },

    #[serde(rename = "image")]
    Image {
        #[serde(flatten)]
        data: ProjectImageResponseModel,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AssetsItem {
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video { .. })
    }

    pub fn as_video(&self) -> Option<&ProjectVideoResponseModel> {
        match self {
            Self::Video { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_video(self) -> Option<ProjectVideoResponseModel> {
        match self {
            Self::Video { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. })
    }

    pub fn as_audio(&self) -> Option<&ProjectExternalAudioResponseModel> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio(self) -> Option<ProjectExternalAudioResponseModel> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn as_image(&self) -> Option<&ProjectImageResponseModel> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_image(self) -> Option<ProjectImageResponseModel> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }
}
