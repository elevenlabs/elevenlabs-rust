pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveTextToDialogueWebsocketMessageMulti {
    AudioChunkMulti(AudioChunkMulti),
    FinalAudioForTurnMulti(FinalAudioForTurnMulti),
    FinalMulti(FinalMulti),
    Error(Error),
}

impl<'de> serde::Deserialize<'de> for ReceiveTextToDialogueWebsocketMessageMulti {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let has_any_key = |keys: &[&str]| {
            value
                .as_object()
                .map(|object| keys.iter().any(|key| object.contains_key(*key)))
                .unwrap_or(false)
        };

        let mut matched_literal = false;

        if value.get("is_final").and_then(serde_json::Value::as_bool) == Some(true) {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<FinalMulti>(value.clone()) {
                return Ok(Self::FinalMulti(parsed));
            }
        }

        if !matched_literal {
            if has_any_key(&["audio", "alignment", "normalized_alignment", "context_id"]) {
                if let Ok(parsed) = serde_json::from_value::<AudioChunkMulti>(value.clone()) {
                    return Ok(Self::AudioChunkMulti(parsed));
                }
            }

            if has_any_key(&["is_final_audio_for_turn", "context_id"]) {
                if let Ok(parsed) = serde_json::from_value::<FinalAudioForTurnMulti>(value.clone())
                {
                    return Ok(Self::FinalAudioForTurnMulti(parsed));
                }
            }

            if let Ok(parsed) = serde_json::from_value::<FinalMulti>(value.clone()) {
                return Ok(Self::FinalMulti(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<Error>(value.clone()) {
                return Ok(Self::Error(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ReceiveTextToDialogueWebsocketMessageMulti",
        ))
    }
}

impl ReceiveTextToDialogueWebsocketMessageMulti {
    pub fn is_audio_chunk_multi(&self) -> bool {
        matches!(self, Self::AudioChunkMulti(_))
    }

    pub fn as_audio_chunk_multi(&self) -> Option<&AudioChunkMulti> {
        match self {
            Self::AudioChunkMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_audio_chunk_multi(self) -> Option<AudioChunkMulti> {
        match self {
            Self::AudioChunkMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_final_audio_for_turn_multi(&self) -> bool {
        matches!(self, Self::FinalAudioForTurnMulti(_))
    }

    pub fn as_final_audio_for_turn_multi(&self) -> Option<&FinalAudioForTurnMulti> {
        match self {
            Self::FinalAudioForTurnMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_final_audio_for_turn_multi(self) -> Option<FinalAudioForTurnMulti> {
        match self {
            Self::FinalAudioForTurnMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_final_multi(&self) -> bool {
        matches!(self, Self::FinalMulti(_))
    }

    pub fn as_final_multi(&self) -> Option<&FinalMulti> {
        match self {
            Self::FinalMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_final_multi(self) -> Option<FinalMulti> {
        match self {
            Self::FinalMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn as_error(&self) -> Option<&Error> {
        match self {
            Self::Error(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<Error> {
        match self {
            Self::Error(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ReceiveTextToDialogueWebsocketMessageMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioChunkMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FinalAudioForTurnMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FinalMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Error(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for ReceiveTextToDialogueWebsocketMessageMulti {
    fn default() -> Self {
        ReceiveTextToDialogueWebsocketMessageMulti::AudioChunkMulti(Default::default())
    }
}
