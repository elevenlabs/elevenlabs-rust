pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SendMessage {
    InitializeConnection(InitializeConnection),
    SendText(SendText),
    CloseConnection(CloseConnection),
}

impl<'de> serde::Deserialize<'de> for SendMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
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

        if value.get("text").and_then(serde_json::Value::as_str) == Some(" ") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<InitializeConnection>(value.clone()) {
                let best = matching_keys(&[
                    "text",
                    "voice_settings",
                    "generation_config",
                    "pronunciation_dictionary_locators",
                    "xi_api_key",
                    "authorization",
                ]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "text",
                    "try_trigger_generation",
                    "voice_settings",
                    "generation_config",
                    "flush",
                ]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<SendText>(value.clone()) {
                        chosen = Some(Self::SendText(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::InitializeConnection(parsed)));
            }
        }

        if value.get("text").and_then(serde_json::Value::as_str) == Some("") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<CloseConnection>(value.clone()) {
                let best = matching_keys(&["text"]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "text",
                    "try_trigger_generation",
                    "voice_settings",
                    "generation_config",
                    "flush",
                ]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<SendText>(value.clone()) {
                        chosen = Some(Self::SendText(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::CloseConnection(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<InitializeConnection>(value.clone()) {
                return Ok(Self::InitializeConnection(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SendText>(value.clone()) {
                return Ok(Self::SendText(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<CloseConnection>(value.clone()) {
                return Ok(Self::CloseConnection(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum SendMessage",
        ))
    }
}

impl SendMessage {
    pub fn is_initialize_connection(&self) -> bool {
        matches!(self, Self::InitializeConnection(_))
    }

    pub fn as_initialize_connection(&self) -> Option<&InitializeConnection> {
        match self {
            Self::InitializeConnection(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_initialize_connection(self) -> Option<InitializeConnection> {
        match self {
            Self::InitializeConnection(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_send_text(&self) -> bool {
        matches!(self, Self::SendText(_))
    }

    pub fn as_send_text(&self) -> Option<&SendText> {
        match self {
            Self::SendText(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_send_text(self) -> Option<SendText> {
        match self {
            Self::SendText(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_close_connection(&self) -> bool {
        matches!(self, Self::CloseConnection(_))
    }

    pub fn as_close_connection(&self) -> Option<&CloseConnection> {
        match self {
            Self::CloseConnection(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_close_connection(self) -> Option<CloseConnection> {
        match self {
            Self::CloseConnection(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for SendMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializeConnection(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SendText(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CloseConnection(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
