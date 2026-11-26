pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketSubscribersChangedEventResponseType {
    SubscribersChanged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketSubscribersChangedEventResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SubscribersChanged => serializer.serialize_str("subscribers_changed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketSubscribersChangedEventResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "subscribers_changed" => Ok(Self::SubscribersChanged),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketSubscribersChangedEventResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubscribersChanged => write!(f, "subscribers_changed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
