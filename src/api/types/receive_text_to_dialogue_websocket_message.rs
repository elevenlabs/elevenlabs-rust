pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveTextToDialogueWebsocketMessage {
    AudioChunk(AudioChunk),
    FinalAudioForTurn(FinalAudioForTurn),
    Final_(Final_),
    Error(Error),
}

impl<'de> serde::Deserialize<'de> for ReceiveTextToDialogueWebsocketMessage {
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
            if let Ok(parsed) = serde_json::from_value::<Final_>(value.clone()) {
                return Ok(Self::Final_(parsed));
            }
        }

        if !matched_literal {
            if has_any_key(&["audio", "alignment", "normalized_alignment"]) {
                if let Ok(parsed) = serde_json::from_value::<AudioChunk>(value.clone()) {
                    return Ok(Self::AudioChunk(parsed));
                }
            }

            if has_any_key(&["is_final_audio_for_turn"]) {
                if let Ok(parsed) = serde_json::from_value::<FinalAudioForTurn>(value.clone()) {
                    return Ok(Self::FinalAudioForTurn(parsed));
                }
            }

            if let Ok(parsed) = serde_json::from_value::<Final_>(value.clone()) {
                return Ok(Self::Final_(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<Error>(value.clone()) {
                return Ok(Self::Error(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ReceiveTextToDialogueWebsocketMessage",
        ))
    }
}

impl ReceiveTextToDialogueWebsocketMessage {
    pub fn is_audio_chunk(&self) -> bool {
        matches!(self, Self::AudioChunk(_))
    }

    pub fn as_audio_chunk(&self) -> Option<&AudioChunk> {
        match self {
            Self::AudioChunk(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_audio_chunk(self) -> Option<AudioChunk> {
        match self {
            Self::AudioChunk(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_final_audio_for_turn(&self) -> bool {
        matches!(self, Self::FinalAudioForTurn(_))
    }

    pub fn as_final_audio_for_turn(&self) -> Option<&FinalAudioForTurn> {
        match self {
            Self::FinalAudioForTurn(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_final_audio_for_turn(self) -> Option<FinalAudioForTurn> {
        match self {
            Self::FinalAudioForTurn(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_final(&self) -> bool {
        matches!(self, Self::Final_(_))
    }

    pub fn as_final(&self) -> Option<&Final_> {
        match self {
            Self::Final_(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_final(self) -> Option<Final_> {
        match self {
            Self::Final_(value) => Some(value),
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

impl fmt::Display for ReceiveTextToDialogueWebsocketMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioChunk(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FinalAudioForTurn(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Final_(value) => write!(
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

impl Default for ReceiveTextToDialogueWebsocketMessage {
    fn default() -> Self {
        ReceiveTextToDialogueWebsocketMessage::AudioChunk(Default::default())
    }
}
