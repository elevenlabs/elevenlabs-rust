pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketStatus {
    Open,
    Pending,
    Waiting,
    Resolved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Waiting => serializer.serialize_str("waiting"),
            Self::Resolved => serializer.serialize_str("resolved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "pending" => Ok(Self::Pending),
            "waiting" => Ok(Self::Waiting),
            "resolved" => Ok(Self::Resolved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Pending => write!(f, "pending"),
            Self::Waiting => write!(f, "waiting"),
            Self::Resolved => write!(f, "resolved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
