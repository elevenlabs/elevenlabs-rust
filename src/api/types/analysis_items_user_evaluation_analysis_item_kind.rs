pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserEvaluationAnalysisItemKind {
    Evaluation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UserEvaluationAnalysisItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Evaluation => serializer.serialize_str("evaluation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UserEvaluationAnalysisItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "evaluation" => Ok(Self::Evaluation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UserEvaluationAnalysisItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evaluation => write!(f, "evaluation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
