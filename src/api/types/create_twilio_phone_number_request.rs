pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTwilioPhoneNumberRequest {
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
    /// Twilio Account SID (starts with `AC`) or API Key SID (starts with `SK`)
    #[serde(default)]
    pub sid: String,
    /// Secret paired with `sid`: the Account Auth Token for an Account SID, or the API Key Secret for an API Key SID
    #[serde(default)]
    pub token: String,
    /// Twilio Account Auth Token, required for API Key imports to validate inbound webhook signatures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_auth_token: Option<String>,
    /// Twilio Additional Region Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_config: Option<RegionConfigRequest>,
    /// Route inbound SMS to ElevenLabs. On by default; set to false to skip SMS configuration for numbers that don't support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sms: Option<bool>,
}

impl CreateTwilioPhoneNumberRequest {
    pub fn builder() -> CreateTwilioPhoneNumberRequestBuilder {
        <CreateTwilioPhoneNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTwilioPhoneNumberRequestBuilder {
    phone_number: Option<String>,
    label: Option<String>,
    supports_inbound: Option<bool>,
    supports_outbound: Option<bool>,
    agent_id: Option<String>,
    sid: Option<String>,
    token: Option<String>,
    account_auth_token: Option<String>,
    region_config: Option<RegionConfigRequest>,
    enable_sms: Option<bool>,
}

impl CreateTwilioPhoneNumberRequestBuilder {
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

    pub fn sid(mut self, value: impl Into<String>) -> Self {
        self.sid = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn account_auth_token(mut self, value: impl Into<String>) -> Self {
        self.account_auth_token = Some(value.into());
        self
    }

    pub fn region_config(mut self, value: RegionConfigRequest) -> Self {
        self.region_config = Some(value);
        self
    }

    pub fn enable_sms(mut self, value: bool) -> Self {
        self.enable_sms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTwilioPhoneNumberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](CreateTwilioPhoneNumberRequestBuilder::phone_number)
    /// - [`label`](CreateTwilioPhoneNumberRequestBuilder::label)
    /// - [`sid`](CreateTwilioPhoneNumberRequestBuilder::sid)
    /// - [`token`](CreateTwilioPhoneNumberRequestBuilder::token)
    pub fn build(self) -> Result<CreateTwilioPhoneNumberRequest, BuildError> {
        Ok(CreateTwilioPhoneNumberRequest {
            phone_number: self
                .phone_number
                .ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            supports_inbound: self.supports_inbound,
            supports_outbound: self.supports_outbound,
            agent_id: self.agent_id,
            sid: self.sid.ok_or_else(|| BuildError::missing_field("sid"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            account_auth_token: self.account_auth_token,
            region_config: self.region_config,
            enable_sms: self.enable_sms,
        })
    }
}
