pub use crate::prelude::*;

/// Status of a single MCP integration/server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IntegrationsItem {
    #[serde(default)]
    pub integration_id: String,
    pub integration_type: IntegrationsItemIntegrationType,
    #[serde(default)]
    pub is_connected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub tool_count: Option<i64>,
}

impl IntegrationsItem {
    pub fn builder() -> IntegrationsItemBuilder {
        <IntegrationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntegrationsItemBuilder {
    integration_id: Option<String>,
    integration_type: Option<IntegrationsItemIntegrationType>,
    is_connected: Option<bool>,
    tool_count: Option<i64>,
}

impl IntegrationsItemBuilder {
    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    pub fn integration_type(mut self, value: IntegrationsItemIntegrationType) -> Self {
        self.integration_type = Some(value);
        self
    }

    pub fn is_connected(mut self, value: bool) -> Self {
        self.is_connected = Some(value);
        self
    }

    pub fn tool_count(mut self, value: i64) -> Self {
        self.tool_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntegrationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`integration_id`](IntegrationsItemBuilder::integration_id)
    /// - [`integration_type`](IntegrationsItemBuilder::integration_type)
    /// - [`is_connected`](IntegrationsItemBuilder::is_connected)
    pub fn build(self) -> Result<IntegrationsItem, BuildError> {
        Ok(IntegrationsItem {
            integration_id: self
                .integration_id
                .ok_or_else(|| BuildError::missing_field("integration_id"))?,
            integration_type: self
                .integration_type
                .ok_or_else(|| BuildError::missing_field("integration_type"))?,
            is_connected: self
                .is_connected
                .ok_or_else(|| BuildError::missing_field("is_connected"))?,
            tool_count: self.tool_count,
        })
    }
}
