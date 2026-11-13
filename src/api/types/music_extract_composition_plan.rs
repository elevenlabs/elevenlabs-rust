pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ExtractCompositionPlan {
    Boolean(bool),
    MusicModelId(MusicModelId),
}

impl ExtractCompositionPlan {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_music_model_id(&self) -> bool {
        matches!(self, Self::MusicModelId(_))
    }

    pub fn as_music_model_id(&self) -> Option<&MusicModelId> {
        match self {
            Self::MusicModelId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_music_model_id(self) -> Option<MusicModelId> {
        match self {
            Self::MusicModelId(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ExtractCompositionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::MusicModelId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for ExtractCompositionPlan {
    fn default() -> Self {
        ExtractCompositionPlan::Boolean(Default::default())
    }
}
