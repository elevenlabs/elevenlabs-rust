pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiIntegrationWebhookToolConfigInputType {
    ApiIntegrationWebhook,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiIntegrationWebhookToolConfigInputType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApiIntegrationWebhook => serializer.serialize_str("api_integration_webhook"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiIntegrationWebhookToolConfigInputType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api_integration_webhook" => Ok(Self::ApiIntegrationWebhook),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiIntegrationWebhookToolConfigInputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiIntegrationWebhook => write!(f, "api_integration_webhook"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
