pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum Outcome {
    #[serde(rename = "closed")]
    Closed {
        #[serde(flatten)]
        data: ClosedOutcome,
    },

    #[serde(rename = "merged")]
    Merged {
        #[serde(flatten)]
        data: MergedOutcome,
    },

    #[serde(rename = "open")]
    Open {
        #[serde(flatten)]
        data: OpenOutcome,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl Outcome {
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed { .. })
    }

    pub fn as_closed(&self) -> Option<&ClosedOutcome> {
        match self {
            Self::Closed { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_closed(self) -> Option<ClosedOutcome> {
        match self {
            Self::Closed { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_merged(&self) -> bool {
        matches!(self, Self::Merged { .. })
    }

    pub fn as_merged(&self) -> Option<&MergedOutcome> {
        match self {
            Self::Merged { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_merged(self) -> Option<MergedOutcome> {
        match self {
            Self::Merged { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open { .. })
    }

    pub fn as_open(&self) -> Option<&OpenOutcome> {
        match self {
            Self::Open { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_open(self) -> Option<OpenOutcome> {
        match self {
            Self::Open { data } => Some(data),
            _ => None,
        }
    }
}
