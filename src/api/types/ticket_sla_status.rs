pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketSlaStatus {
    Overdue,
    OnTrack,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketSlaStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Overdue => serializer.serialize_str("overdue"),
            Self::OnTrack => serializer.serialize_str("on_track"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketSlaStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "overdue" => Ok(Self::Overdue),
            "on_track" => Ok(Self::OnTrack),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketSlaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overdue => write!(f, "overdue"),
            Self::OnTrack => write!(f, "on_track"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
