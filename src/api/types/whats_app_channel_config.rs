pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhatsAppChannelConfig {
    /// The WhatsApp phone number to call or message from.
    #[serde(default)]
    pub whatsapp_phone_number_id: String,
    pub mode_config: WhatsAppChannelConfigModeConfig,
}

impl WhatsAppChannelConfig {
    pub fn builder() -> WhatsAppChannelConfigBuilder {
        <WhatsAppChannelConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppChannelConfigBuilder {
    whatsapp_phone_number_id: Option<String>,
    mode_config: Option<WhatsAppChannelConfigModeConfig>,
}

impl WhatsAppChannelConfigBuilder {
    pub fn whatsapp_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_phone_number_id = Some(value.into());
        self
    }

    pub fn mode_config(mut self, value: WhatsAppChannelConfigModeConfig) -> Self {
        self.mode_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppChannelConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_phone_number_id`](WhatsAppChannelConfigBuilder::whatsapp_phone_number_id)
    /// - [`mode_config`](WhatsAppChannelConfigBuilder::mode_config)
    pub fn build(self) -> Result<WhatsAppChannelConfig, BuildError> {
        Ok(WhatsAppChannelConfig {
            whatsapp_phone_number_id: self
                .whatsapp_phone_number_id
                .ok_or_else(|| BuildError::missing_field("whatsapp_phone_number_id"))?,
            mode_config: self
                .mode_config
                .ok_or_else(|| BuildError::missing_field("mode_config"))?,
        })
    }
}
