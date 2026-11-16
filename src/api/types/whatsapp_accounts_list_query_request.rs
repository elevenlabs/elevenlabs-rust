pub use crate::prelude::*;

/// Query parameters for List Whatsapp Accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WhatsappAccountsListQueryRequest {
    /// Filter by assigned agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl WhatsappAccountsListQueryRequest {
    pub fn builder() -> WhatsappAccountsListQueryRequestBuilder {
        <WhatsappAccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsappAccountsListQueryRequestBuilder {
    agent_id: Option<String>,
}

impl WhatsappAccountsListQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsappAccountsListQueryRequest`].
    pub fn build(self) -> Result<WhatsappAccountsListQueryRequest, BuildError> {
        Ok(WhatsappAccountsListQueryRequest {
            agent_id: self.agent_id,
        })
    }
}
