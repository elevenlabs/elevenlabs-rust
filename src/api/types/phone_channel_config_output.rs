pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PhoneChannelConfigOutput {
    /// The phone number to call from.
    #[serde(default)]
    pub agent_phone_number_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_config: Option<PhoneVoiceConfigOutput>,
}

impl PhoneChannelConfigOutput {
    pub fn builder() -> PhoneChannelConfigOutputBuilder {
        <PhoneChannelConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneChannelConfigOutputBuilder {
    agent_phone_number_id: Option<String>,
    mode_config: Option<PhoneVoiceConfigOutput>,
}

impl PhoneChannelConfigOutputBuilder {
    pub fn agent_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.agent_phone_number_id = Some(value.into());
        self
    }

    pub fn mode_config(mut self, value: PhoneVoiceConfigOutput) -> Self {
        self.mode_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PhoneChannelConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_phone_number_id`](PhoneChannelConfigOutputBuilder::agent_phone_number_id)
    pub fn build(self) -> Result<PhoneChannelConfigOutput, BuildError> {
        Ok(PhoneChannelConfigOutput {
            agent_phone_number_id: self
                .agent_phone_number_id
                .ok_or_else(|| BuildError::missing_field("agent_phone_number_id"))?,
            mode_config: self.mode_config,
        })
    }
}
