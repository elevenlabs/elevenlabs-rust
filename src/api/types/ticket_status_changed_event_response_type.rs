pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketStatusChangedEventResponseType {
    StatusChanged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketStatusChangedEventResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StatusChanged => serializer.serialize_str("status_changed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketStatusChangedEventResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "status_changed" => Ok(Self::StatusChanged),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketStatusChangedEventResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StatusChanged => write!(f, "status_changed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
