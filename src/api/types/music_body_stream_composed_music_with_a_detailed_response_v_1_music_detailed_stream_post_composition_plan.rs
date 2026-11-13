pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPostCompositionPlan {
    MusicPrompt(MusicPrompt),
    CompositionPlan2(CompositionPlan2),
}

impl BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPostCompositionPlan {
    pub fn is_music_prompt(&self) -> bool {
        matches!(self, Self::MusicPrompt(_))
    }

    pub fn as_music_prompt(&self) -> Option<&MusicPrompt> {
        match self {
            Self::MusicPrompt(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_music_prompt(self) -> Option<MusicPrompt> {
        match self {
            Self::MusicPrompt(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_composition_plan2(&self) -> bool {
        matches!(self, Self::CompositionPlan2(_))
    }

    pub fn as_composition_plan2(&self) -> Option<&CompositionPlan2> {
        match self {
            Self::CompositionPlan2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_composition_plan2(self) -> Option<CompositionPlan2> {
        match self {
            Self::CompositionPlan2(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display
    for BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPostCompositionPlan
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MusicPrompt(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CompositionPlan2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
