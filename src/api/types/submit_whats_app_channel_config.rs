pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitWhatsAppChannelConfig {
    /// The WhatsApp phone number to call or message from.
    #[serde(default)]
    pub whatsapp_phone_number_id: String,
    pub mode_config: SubmitWhatsAppChannelConfigModeConfig,
}

impl SubmitWhatsAppChannelConfig {
    pub fn builder() -> SubmitWhatsAppChannelConfigBuilder {
        <SubmitWhatsAppChannelConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitWhatsAppChannelConfigBuilder {
    whatsapp_phone_number_id: Option<String>,
    mode_config: Option<SubmitWhatsAppChannelConfigModeConfig>,
}

impl SubmitWhatsAppChannelConfigBuilder {
    pub fn whatsapp_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_phone_number_id = Some(value.into());
        self
    }

    pub fn mode_config(mut self, value: SubmitWhatsAppChannelConfigModeConfig) -> Self {
        self.mode_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitWhatsAppChannelConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_phone_number_id`](SubmitWhatsAppChannelConfigBuilder::whatsapp_phone_number_id)
    /// - [`mode_config`](SubmitWhatsAppChannelConfigBuilder::mode_config)
    pub fn build(self) -> Result<SubmitWhatsAppChannelConfig, BuildError> {
        Ok(SubmitWhatsAppChannelConfig {
            whatsapp_phone_number_id: self
                .whatsapp_phone_number_id
                .ok_or_else(|| BuildError::missing_field("whatsapp_phone_number_id"))?,
            mode_config: self
                .mode_config
                .ok_or_else(|| BuildError::missing_field("mode_config"))?,
        })
    }
}
