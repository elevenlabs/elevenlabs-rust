pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WidgetConfigAvatar {
    #[serde(rename = "orb")]
    Orb {
        #[serde(flatten)]
        data: OrbAvatar,
    },

    #[serde(rename = "url")]
    Url {
        #[serde(flatten)]
        data: UrlAvatar,
    },

    #[serde(rename = "image")]
    Image {
        #[serde(flatten)]
        data: ImageAvatar,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WidgetConfigAvatar {
    pub fn is_orb(&self) -> bool {
        matches!(self, Self::Orb { .. })
    }

    pub fn as_orb(&self) -> Option<&OrbAvatar> {
        match self {
            Self::Orb { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_orb(self) -> Option<OrbAvatar> {
        match self {
            Self::Orb { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    pub fn as_url(&self) -> Option<&UrlAvatar> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_url(self) -> Option<UrlAvatar> {
        match self {
            Self::Url { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn as_image(&self) -> Option<&ImageAvatar> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_image(self) -> Option<ImageAvatar> {
        match self {
            Self::Image { data } => Some(data),
            _ => None,
        }
    }
}
