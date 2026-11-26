pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketDeletedEventResponseType {
    TicketDeleted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketDeletedEventResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TicketDeleted => serializer.serialize_str("ticket_deleted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketDeletedEventResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ticket_deleted" => Ok(Self::TicketDeleted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketDeletedEventResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TicketDeleted => write!(f, "ticket_deleted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
