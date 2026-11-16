pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BodyCreateDubbingProjectV1DubbingProjectPostModelId {
    DubbingModels(DubbingModels),
    String(String),
}

impl BodyCreateDubbingProjectV1DubbingProjectPostModelId {
    pub fn is_dubbing_models(&self) -> bool {
        matches!(self, Self::DubbingModels(_))
    }

    pub fn as_dubbing_models(&self) -> Option<&DubbingModels> {
        match self {
            Self::DubbingModels(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_dubbing_models(self) -> Option<DubbingModels> {
        match self {
            Self::DubbingModels(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for BodyCreateDubbingProjectV1DubbingProjectPostModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DubbingModels(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::String(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for BodyCreateDubbingProjectV1DubbingProjectPostModelId {
    fn default() -> Self {
        BodyCreateDubbingProjectV1DubbingProjectPostModelId::String(Default::default())
    }
}
