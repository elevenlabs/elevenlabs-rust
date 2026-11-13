pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScribeUnacceptedTermsErrorMessageType {
    UnacceptedTerms,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScribeUnacceptedTermsErrorMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UnacceptedTerms => serializer.serialize_str("unaccepted_terms"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScribeUnacceptedTermsErrorMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unaccepted_terms" => Ok(Self::UnacceptedTerms),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScribeUnacceptedTermsErrorMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnacceptedTerms => write!(f, "unaccepted_terms"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
