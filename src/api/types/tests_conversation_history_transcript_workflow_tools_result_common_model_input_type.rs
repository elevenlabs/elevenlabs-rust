pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationHistoryTranscriptWorkflowToolsResultCommonModelInputType {
    Workflow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationHistoryTranscriptWorkflowToolsResultCommonModelInputType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Workflow => serializer.serialize_str("workflow"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for ConversationHistoryTranscriptWorkflowToolsResultCommonModelInputType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "workflow" => Ok(Self::Workflow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationHistoryTranscriptWorkflowToolsResultCommonModelInputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workflow => write!(f, "workflow"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
