pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketAssigneeChangedEventResponseType {
    AssigneeChanged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketAssigneeChangedEventResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AssigneeChanged => serializer.serialize_str("assignee_changed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketAssigneeChangedEventResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "assignee_changed" => Ok(Self::AssigneeChanged),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketAssigneeChangedEventResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssigneeChanged => write!(f, "assignee_changed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
