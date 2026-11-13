pub use crate::prelude::*;

/// Query parameters for List Phone Numbers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PhoneNumbersListQueryRequest {
    /// Filter by telephony provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<TelephonyProvider>,
    /// Filter by assigned agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter by assigned branch ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl PhoneNumbersListQueryRequest {
    pub fn builder() -> PhoneNumbersListQueryRequestBuilder {
        <PhoneNumbersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneNumbersListQueryRequestBuilder {
    provider: Option<TelephonyProvider>,
    agent_id: Option<String>,
    branch_id: Option<String>,
}

impl PhoneNumbersListQueryRequestBuilder {
    pub fn provider(mut self, value: TelephonyProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneNumbersListQueryRequest`].
    pub fn build(self) -> Result<PhoneNumbersListQueryRequest, BuildError> {
        Ok(PhoneNumbersListQueryRequest {
            provider: self.provider,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
        })
    }
}
