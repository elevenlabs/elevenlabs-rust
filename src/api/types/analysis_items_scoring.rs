pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode")]
#[non_exhaustive]
pub enum Scoring {
    #[serde(rename = "binary")]
    Binary {
        #[serde(flatten)]
        data: BinaryScoring,
    },

    #[serde(rename = "numeric_uniform")]
    NumericUniform {
        #[serde(flatten)]
        data: NumericUniformScoring,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl Scoring {
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary { .. })
    }

    pub fn as_binary(&self) -> Option<&BinaryScoring> {
        match self {
            Self::Binary { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_binary(self) -> Option<BinaryScoring> {
        match self {
            Self::Binary { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_numeric_uniform(&self) -> bool {
        matches!(self, Self::NumericUniform { .. })
    }

    pub fn as_numeric_uniform(&self) -> Option<&NumericUniformScoring> {
        match self {
            Self::NumericUniform { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_numeric_uniform(self) -> Option<NumericUniformScoring> {
        match self {
            Self::NumericUniform { data } => Some(data),
            _ => None,
        }
    }
}
