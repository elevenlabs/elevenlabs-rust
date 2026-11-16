pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoicesNotPermittedWarningType {
    VoicesNotPermitted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoicesNotPermittedWarningType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VoicesNotPermitted => serializer.serialize_str("voices_not_permitted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoicesNotPermittedWarningType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "voices_not_permitted" => Ok(Self::VoicesNotPermitted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoicesNotPermittedWarningType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VoicesNotPermitted => write!(f, "voices_not_permitted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
