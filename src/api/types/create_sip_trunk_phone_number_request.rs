pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSipTrunkPhoneNumberRequest {
    /// Phone number
    #[serde(default)]
    pub phone_number: String,
    /// Label for the phone number
    #[serde(default)]
    pub label: String,
    /// This field is deprecated and will be removed in the future. Whether this phone number supports inbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inbound: Option<bool>,
    /// This field is deprecated and will be removed in the future. Whether this phone number supports outbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_outbound: Option<bool>,
    /// Agent ID to assign the phone number to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>,
}

impl CreateSipTrunkPhoneNumberRequest {
    pub fn builder() -> CreateSipTrunkPhoneNumberRequestBuilder {
        <CreateSipTrunkPhoneNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSipTrunkPhoneNumberRequestBuilder {
    phone_number: Option<String>,
    label: Option<String>,
    supports_inbound: Option<bool>,
    supports_outbound: Option<bool>,
    agent_id: Option<String>,
    inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>,
    outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>,
}

impl CreateSipTrunkPhoneNumberRequestBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn supports_inbound(mut self, value: bool) -> Self {
        self.supports_inbound = Some(value);
        self
    }

    pub fn supports_outbound(mut self, value: bool) -> Self {
        self.supports_outbound = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn inbound_trunk_config(mut self, value: InboundSipTrunkConfigRequestModel) -> Self {
        self.inbound_trunk_config = Some(value);
        self
    }

    pub fn outbound_trunk_config(mut self, value: OutboundSipTrunkConfigRequestModel) -> Self {
        self.outbound_trunk_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSipTrunkPhoneNumberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](CreateSipTrunkPhoneNumberRequestBuilder::phone_number)
    /// - [`label`](CreateSipTrunkPhoneNumberRequestBuilder::label)
    pub fn build(self) -> Result<CreateSipTrunkPhoneNumberRequest, BuildError> {
        Ok(CreateSipTrunkPhoneNumberRequest {
            phone_number: self
                .phone_number
                .ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            supports_inbound: self.supports_inbound,
            supports_outbound: self.supports_outbound,
            agent_id: self.agent_id,
            inbound_trunk_config: self.inbound_trunk_config,
            outbound_trunk_config: self.outbound_trunk_config,
        })
    }
}
