pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateExotelPhoneNumberRequest {
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
    /// Exotel Account SID
    #[serde(default)]
    pub account_sid: String,
    /// Exotel API Key
    #[serde(default)]
    pub api_key: String,
    /// Exotel API Token
    #[serde(default)]
    pub api_token: String,
    pub api_subdomain: ExotelApiSubdomain,
    /// Exotel applet identifier used in Calls/connect
    #[serde(default)]
    pub app_id: String,
    /// Optional full applet URL override. Defaults to Exotel start_voice URL derived from account SID and app ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applet_url: Option<String>,
}

impl CreateExotelPhoneNumberRequest {
    pub fn builder() -> CreateExotelPhoneNumberRequestBuilder {
        <CreateExotelPhoneNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExotelPhoneNumberRequestBuilder {
    phone_number: Option<String>,
    label: Option<String>,
    supports_inbound: Option<bool>,
    supports_outbound: Option<bool>,
    agent_id: Option<String>,
    account_sid: Option<String>,
    api_key: Option<String>,
    api_token: Option<String>,
    api_subdomain: Option<ExotelApiSubdomain>,
    app_id: Option<String>,
    applet_url: Option<String>,
}

impl CreateExotelPhoneNumberRequestBuilder {
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

    pub fn account_sid(mut self, value: impl Into<String>) -> Self {
        self.account_sid = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn api_token(mut self, value: impl Into<String>) -> Self {
        self.api_token = Some(value.into());
        self
    }

    pub fn api_subdomain(mut self, value: ExotelApiSubdomain) -> Self {
        self.api_subdomain = Some(value);
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn applet_url(mut self, value: impl Into<String>) -> Self {
        self.applet_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExotelPhoneNumberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](CreateExotelPhoneNumberRequestBuilder::phone_number)
    /// - [`label`](CreateExotelPhoneNumberRequestBuilder::label)
    /// - [`account_sid`](CreateExotelPhoneNumberRequestBuilder::account_sid)
    /// - [`api_key`](CreateExotelPhoneNumberRequestBuilder::api_key)
    /// - [`api_token`](CreateExotelPhoneNumberRequestBuilder::api_token)
    /// - [`api_subdomain`](CreateExotelPhoneNumberRequestBuilder::api_subdomain)
    /// - [`app_id`](CreateExotelPhoneNumberRequestBuilder::app_id)
    pub fn build(self) -> Result<CreateExotelPhoneNumberRequest, BuildError> {
        Ok(CreateExotelPhoneNumberRequest {
            phone_number: self
                .phone_number
                .ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            supports_inbound: self.supports_inbound,
            supports_outbound: self.supports_outbound,
            agent_id: self.agent_id,
            account_sid: self
                .account_sid
                .ok_or_else(|| BuildError::missing_field("account_sid"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            api_token: self
                .api_token
                .ok_or_else(|| BuildError::missing_field("api_token"))?,
            api_subdomain: self
                .api_subdomain
                .ok_or_else(|| BuildError::missing_field("api_subdomain"))?,
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            applet_url: self.applet_url,
        })
    }
}
