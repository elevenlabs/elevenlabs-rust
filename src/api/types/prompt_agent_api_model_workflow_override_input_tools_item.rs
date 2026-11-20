pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PromptAgentApiModelWorkflowOverrideInputToolsItem {
    WebhookToolConfigInput(WebhookToolConfigInput),
    ClientToolConfigInput(ClientToolConfigInput),
    SystemToolConfigInput(SystemToolConfigInput),
    ApiIntegrationWebhookToolConfigInput(ApiIntegrationWebhookToolConfigInput),
    SmbToolConfig(SmbToolConfig),
}

impl<'de> serde::Deserialize<'de> for PromptAgentApiModelWorkflowOverrideInputToolsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("webhook") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<WebhookToolConfigInput>(value.clone()) {
                return Ok(Self::WebhookToolConfigInput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("client") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ClientToolConfigInput>(value.clone()) {
                return Ok(Self::ClientToolConfigInput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("system") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<SystemToolConfigInput>(value.clone()) {
                return Ok(Self::SystemToolConfigInput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("api_integration_webhook")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ApiIntegrationWebhookToolConfigInput>(value.clone())
            {
                return Ok(Self::ApiIntegrationWebhookToolConfigInput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("smb") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<SmbToolConfig>(value.clone()) {
                return Ok(Self::SmbToolConfig(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<WebhookToolConfigInput>(value.clone()) {
                return Ok(Self::WebhookToolConfigInput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ClientToolConfigInput>(value.clone()) {
                return Ok(Self::ClientToolConfigInput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SystemToolConfigInput>(value.clone()) {
                return Ok(Self::SystemToolConfigInput(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<ApiIntegrationWebhookToolConfigInput>(value.clone())
            {
                return Ok(Self::ApiIntegrationWebhookToolConfigInput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SmbToolConfig>(value.clone()) {
                return Ok(Self::SmbToolConfig(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum PromptAgentApiModelWorkflowOverrideInputToolsItem",
        ))
    }
}

impl PromptAgentApiModelWorkflowOverrideInputToolsItem {
    pub fn is_webhook_tool_config_input(&self) -> bool {
        matches!(self, Self::WebhookToolConfigInput(_))
    }

    pub fn as_webhook_tool_config_input(&self) -> Option<&WebhookToolConfigInput> {
        match self {
            Self::WebhookToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_webhook_tool_config_input(self) -> Option<WebhookToolConfigInput> {
        match self {
            Self::WebhookToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_client_tool_config_input(&self) -> bool {
        matches!(self, Self::ClientToolConfigInput(_))
    }

    pub fn as_client_tool_config_input(&self) -> Option<&ClientToolConfigInput> {
        match self {
            Self::ClientToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_client_tool_config_input(self) -> Option<ClientToolConfigInput> {
        match self {
            Self::ClientToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_system_tool_config_input(&self) -> bool {
        matches!(self, Self::SystemToolConfigInput(_))
    }

    pub fn as_system_tool_config_input(&self) -> Option<&SystemToolConfigInput> {
        match self {
            Self::SystemToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_system_tool_config_input(self) -> Option<SystemToolConfigInput> {
        match self {
            Self::SystemToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_api_integration_webhook_tool_config_input(&self) -> bool {
        matches!(self, Self::ApiIntegrationWebhookToolConfigInput(_))
    }

    pub fn as_api_integration_webhook_tool_config_input(
        &self,
    ) -> Option<&ApiIntegrationWebhookToolConfigInput> {
        match self {
            Self::ApiIntegrationWebhookToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_api_integration_webhook_tool_config_input(
        self,
    ) -> Option<ApiIntegrationWebhookToolConfigInput> {
        match self {
            Self::ApiIntegrationWebhookToolConfigInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_smb_tool_config(&self) -> bool {
        matches!(self, Self::SmbToolConfig(_))
    }

    pub fn as_smb_tool_config(&self) -> Option<&SmbToolConfig> {
        match self {
            Self::SmbToolConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_smb_tool_config(self) -> Option<SmbToolConfig> {
        match self {
            Self::SmbToolConfig(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PromptAgentApiModelWorkflowOverrideInputToolsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebhookToolConfigInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ClientToolConfigInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SystemToolConfigInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ApiIntegrationWebhookToolConfigInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SmbToolConfig(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
