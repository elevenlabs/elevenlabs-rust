pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveMessageMulti {
    AudioOutputMulti(AudioOutputMulti),
    FinalOutputMulti(FinalOutputMulti),
}

impl<'de> serde::Deserialize<'de> for ReceiveMessageMulti {
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
            if let Ok(parsed) = serde_json::from_value::<FinalOutputMulti>(value.clone()) {
                let best = matching_keys(&["is_final", "context_id"]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "audio",
                    "is_final",
                    "normalized_alignment",
                    "alignment",
                    "context_id",
                ]);
                if score > best {
                    if let Ok(alternative) =
                        serde_json::from_value::<AudioOutputMulti>(value.clone())
                    {
                        chosen = Some(Self::AudioOutputMulti(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::FinalOutputMulti(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<AudioOutputMulti>(value.clone()) {
                return Ok(Self::AudioOutputMulti(parsed));
            }

            if has_any_key(&["is_final", "context_id"]) {
                if let Ok(parsed) = serde_json::from_value::<FinalOutputMulti>(value.clone()) {
                    return Ok(Self::FinalOutputMulti(parsed));
                }
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ReceiveMessageMulti",
        ))
    }
}

impl ReceiveMessageMulti {
    pub fn is_audio_output_multi(&self) -> bool {
        matches!(self, Self::AudioOutputMulti(_))
    }

    pub fn as_audio_output_multi(&self) -> Option<&AudioOutputMulti> {
        match self {
            Self::AudioOutputMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_audio_output_multi(self) -> Option<AudioOutputMulti> {
        match self {
            Self::AudioOutputMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_final_output_multi(&self) -> bool {
        matches!(self, Self::FinalOutputMulti(_))
    }

    pub fn as_final_output_multi(&self) -> Option<&FinalOutputMulti> {
        match self {
            Self::FinalOutputMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_final_output_multi(self) -> Option<FinalOutputMulti> {
        match self {
            Self::FinalOutputMulti(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ReceiveMessageMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioOutputMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FinalOutputMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for ReceiveMessageMulti {
    fn default() -> Self {
        ReceiveMessageMulti::FinalOutputMulti(Default::default())
    }
}
