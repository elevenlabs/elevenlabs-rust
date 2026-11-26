pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SlaMetric {
    FirstResponse,
    Resolution,
    NextResponse,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SlaMetric {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FirstResponse => serializer.serialize_str("first_response"),
            Self::Resolution => serializer.serialize_str("resolution"),
            Self::NextResponse => serializer.serialize_str("next_response"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SlaMetric {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "first_response" => Ok(Self::FirstResponse),
            "resolution" => Ok(Self::Resolution),
            "next_response" => Ok(Self::NextResponse),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SlaMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstResponse => write!(f, "first_response"),
            Self::Resolution => write!(f, "resolution"),
            Self::NextResponse => write!(f, "next_response"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
