pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SendMessageMulti {
    InitializeConnectionMulti(InitializeConnectionMulti),
    InitialiseContext(InitialiseContext),
    SendTextMulti(SendTextMulti),
    FlushContext(FlushContext),
    CloseContext(CloseContext),
    CloseSocket(CloseSocket),
    KeepContextAlive(KeepContextAlive),
}

impl<'de> serde::Deserialize<'de> for SendMessageMulti {
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

        if value.get("text").and_then(serde_json::Value::as_str) == Some(" ") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<InitializeConnectionMulti>(value.clone()) {
                let mut best = matching_keys(&[
                    "text",
                    "voice_settings",
                    "generation_config",
                    "pronunciation_dictionary_locators",
                    "xi_api_key",
                    "authorization",
                    "context_id",
                ]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "text",
                    "voice_settings",
                    "generation_config",
                    "pronunciation_dictionary_locators",
                    "xi_api_key",
                    "authorization",
                    "context_id",
                ]);
                if score > best {
                    if let Ok(alternative) =
                        serde_json::from_value::<InitialiseContext>(value.clone())
                    {
                        best = score;
                        chosen = Some(Self::InitialiseContext(alternative));
                    }
                }
                let score = matching_keys(&["text", "context_id", "flush"]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<SendTextMulti>(value.clone())
                    {
                        best = score;
                        chosen = Some(Self::SendTextMulti(alternative));
                    }
                }
                let score = matching_keys(&["context_id", "text", "flush"]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<FlushContext>(value.clone()) {
                        chosen = Some(Self::FlushContext(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::InitializeConnectionMulti(parsed)));
            }
        }

        if value.get("text").and_then(serde_json::Value::as_str) == Some("") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<KeepContextAlive>(value.clone()) {
                let mut best = matching_keys(&["text", "context_id"]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "text",
                    "voice_settings",
                    "generation_config",
                    "pronunciation_dictionary_locators",
                    "xi_api_key",
                    "authorization",
                    "context_id",
                ]);
                if score > best {
                    if let Ok(alternative) =
                        serde_json::from_value::<InitialiseContext>(value.clone())
                    {
                        best = score;
                        chosen = Some(Self::InitialiseContext(alternative));
                    }
                }
                let score = matching_keys(&["text", "context_id", "flush"]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<SendTextMulti>(value.clone())
                    {
                        best = score;
                        chosen = Some(Self::SendTextMulti(alternative));
                    }
                }
                let score = matching_keys(&["context_id", "text", "flush"]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<FlushContext>(value.clone()) {
                        chosen = Some(Self::FlushContext(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::KeepContextAlive(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<InitializeConnectionMulti>(value.clone()) {
                return Ok(Self::InitializeConnectionMulti(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<InitialiseContext>(value.clone()) {
                return Ok(Self::InitialiseContext(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SendTextMulti>(value.clone()) {
                return Ok(Self::SendTextMulti(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<FlushContext>(value.clone()) {
                return Ok(Self::FlushContext(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<CloseContext>(value.clone()) {
                return Ok(Self::CloseContext(parsed));
            }

            if has_any_key(&["close_socket"]) {
                if let Ok(parsed) = serde_json::from_value::<CloseSocket>(value.clone()) {
                    return Ok(Self::CloseSocket(parsed));
                }
            }

            if let Ok(parsed) = serde_json::from_value::<KeepContextAlive>(value.clone()) {
                return Ok(Self::KeepContextAlive(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum SendMessageMulti",
        ))
    }
}

impl SendMessageMulti {
    pub fn is_initialize_connection_multi(&self) -> bool {
        matches!(self, Self::InitializeConnectionMulti(_))
    }

    pub fn as_initialize_connection_multi(&self) -> Option<&InitializeConnectionMulti> {
        match self {
            Self::InitializeConnectionMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_initialize_connection_multi(self) -> Option<InitializeConnectionMulti> {
        match self {
            Self::InitializeConnectionMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_initialise_context(&self) -> bool {
        matches!(self, Self::InitialiseContext(_))
    }

    pub fn as_initialise_context(&self) -> Option<&InitialiseContext> {
        match self {
            Self::InitialiseContext(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_initialise_context(self) -> Option<InitialiseContext> {
        match self {
            Self::InitialiseContext(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_send_text_multi(&self) -> bool {
        matches!(self, Self::SendTextMulti(_))
    }

    pub fn as_send_text_multi(&self) -> Option<&SendTextMulti> {
        match self {
            Self::SendTextMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_send_text_multi(self) -> Option<SendTextMulti> {
        match self {
            Self::SendTextMulti(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_flush_context(&self) -> bool {
        matches!(self, Self::FlushContext(_))
    }

    pub fn as_flush_context(&self) -> Option<&FlushContext> {
        match self {
            Self::FlushContext(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_flush_context(self) -> Option<FlushContext> {
        match self {
            Self::FlushContext(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_close_context(&self) -> bool {
        matches!(self, Self::CloseContext(_))
    }

    pub fn as_close_context(&self) -> Option<&CloseContext> {
        match self {
            Self::CloseContext(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_close_context(self) -> Option<CloseContext> {
        match self {
            Self::CloseContext(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_close_socket(&self) -> bool {
        matches!(self, Self::CloseSocket(_))
    }

    pub fn as_close_socket(&self) -> Option<&CloseSocket> {
        match self {
            Self::CloseSocket(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_close_socket(self) -> Option<CloseSocket> {
        match self {
            Self::CloseSocket(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_keep_context_alive(&self) -> bool {
        matches!(self, Self::KeepContextAlive(_))
    }

    pub fn as_keep_context_alive(&self) -> Option<&KeepContextAlive> {
        match self {
            Self::KeepContextAlive(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_keep_context_alive(self) -> Option<KeepContextAlive> {
        match self {
            Self::KeepContextAlive(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for SendMessageMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializeConnectionMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::InitialiseContext(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SendTextMulti(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FlushContext(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CloseContext(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CloseSocket(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::KeepContextAlive(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for SendMessageMulti {
    fn default() -> Self {
        SendMessageMulti::CloseSocket(Default::default())
    }
}
