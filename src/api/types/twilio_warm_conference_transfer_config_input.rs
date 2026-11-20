pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwilioWarmConferenceTransferConfigInput {
    pub transfer_destination: TwilioWarmConferenceTransferConfigInputTransferDestination,
    /// DTMF digits to send after the call connects (e.g. 'ww1234' for an extension). Static or a dynamic variable reference. Use 'w' for a 0.5s pause.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<TwilioWarmConferenceTransferConfigInputPostDialDigits>,
}

impl TwilioWarmConferenceTransferConfigInput {
    pub fn builder() -> TwilioWarmConferenceTransferConfigInputBuilder {
        <TwilioWarmConferenceTransferConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioWarmConferenceTransferConfigInputBuilder {
    transfer_destination: Option<TwilioWarmConferenceTransferConfigInputTransferDestination>,
    post_dial_digits: Option<TwilioWarmConferenceTransferConfigInputPostDialDigits>,
}

impl TwilioWarmConferenceTransferConfigInputBuilder {
    pub fn transfer_destination(
        mut self,
        value: TwilioWarmConferenceTransferConfigInputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn post_dial_digits(
        mut self,
        value: TwilioWarmConferenceTransferConfigInputPostDialDigits,
    ) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioWarmConferenceTransferConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](TwilioWarmConferenceTransferConfigInputBuilder::transfer_destination)
    pub fn build(self) -> Result<TwilioWarmConferenceTransferConfigInput, BuildError> {
        Ok(TwilioWarmConferenceTransferConfigInput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            post_dial_digits: self.post_dial_digits,
        })
    }
}
