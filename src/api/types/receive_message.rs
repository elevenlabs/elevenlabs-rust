pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveMessage {
    AudioOutput(AudioOutput),
    FinalOutput(FinalOutput),
}

impl<'de> serde::Deserialize<'de> for ReceiveMessage {
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
        let matching_keys = |keys: &[&str]| {
            value
                .as_object()
                .map(|object| {
                    object
                        .keys()
                        .filter(|key| keys.contains(&key.as_str()))
                        .count()
                })
                .unwrap_or(0)
        };

        let mut matched_literal = false;

        if value.get("is_final").and_then(serde_json::Value::as_bool) == Some(true) {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<FinalOutput>(value.clone()) {
                let best = matching_keys(&["is_final"]);
                let mut chosen = None;
                let score =
                    matching_keys(&["audio", "is_final", "normalized_alignment", "alignment"]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<AudioOutput>(value.clone()) {
                        chosen = Some(Self::AudioOutput(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::FinalOutput(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<AudioOutput>(value.clone()) {
                return Ok(Self::AudioOutput(parsed));
            }

            if has_any_key(&["is_final"]) {
                if let Ok(parsed) = serde_json::from_value::<FinalOutput>(value.clone()) {
                    return Ok(Self::FinalOutput(parsed));
                }
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ReceiveMessage",
        ))
    }
}

impl ReceiveMessage {
    pub fn is_audio_output(&self) -> bool {
        matches!(self, Self::AudioOutput(_))
    }

    pub fn as_audio_output(&self) -> Option<&AudioOutput> {
        match self {
            Self::AudioOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_audio_output(self) -> Option<AudioOutput> {
        match self {
            Self::AudioOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_final_output(&self) -> bool {
        matches!(self, Self::FinalOutput(_))
    }

    pub fn as_final_output(&self) -> Option<&FinalOutput> {
        match self {
            Self::FinalOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_final_output(self) -> Option<FinalOutput> {
        match self {
            Self::FinalOutput(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ReceiveMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FinalOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for ReceiveMessage {
    fn default() -> Self {
        ReceiveMessage::FinalOutput(Default::default())
    }
}
