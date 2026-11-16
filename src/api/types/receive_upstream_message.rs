pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ReceiveUpstreamMessage {
    #[serde(rename = "init")]
    Init {
        #[serde(flatten)]
        data: Init,
    },

    #[serde(rename = "user_transcript")]
    UserTranscript {
        #[serde(flatten)]
        data: UserTranscript,
    },

    #[serde(rename = "ping")]
    Ping {
        #[serde(flatten)]
        data: Ping,
    },

    #[serde(rename = "close")]
    Close {
        #[serde(flatten)]
        data: Close,
    },

    #[serde(rename = "error")]
    Error {
        #[serde(flatten)]
        data: SpeechEngineError,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ReceiveUpstreamMessage {
    pub fn is_init(&self) -> bool {
        matches!(self, Self::Init { .. })
    }

    pub fn as_init(&self) -> Option<&Init> {
        match self {
            Self::Init { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_init(self) -> Option<Init> {
        match self {
            Self::Init { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_user_transcript(&self) -> bool {
        matches!(self, Self::UserTranscript { .. })
    }

    pub fn as_user_transcript(&self) -> Option<&UserTranscript> {
        match self {
            Self::UserTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_user_transcript(self) -> Option<UserTranscript> {
        match self {
            Self::UserTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_ping(&self) -> bool {
        matches!(self, Self::Ping { .. })
    }

    pub fn as_ping(&self) -> Option<&Ping> {
        match self {
            Self::Ping { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_ping(self) -> Option<Ping> {
        match self {
            Self::Ping { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_close(&self) -> bool {
        matches!(self, Self::Close { .. })
    }

    pub fn as_close(&self) -> Option<&Close> {
        match self {
            Self::Close { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_close(self) -> Option<Close> {
        match self {
            Self::Close { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn as_error(&self) -> Option<&SpeechEngineError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<SpeechEngineError> {
        match self {
            Self::Error { data } => Some(data),
            _ => None,
        }
    }
}
