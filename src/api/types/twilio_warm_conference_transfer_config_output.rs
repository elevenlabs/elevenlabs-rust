pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwilioWarmConferenceTransferConfigOutput {
    pub transfer_destination: TwilioWarmConferenceTransferConfigOutputTransferDestination,
    /// DTMF digits to send after the call connects (e.g. 'ww1234' for an extension). Static or a dynamic variable reference. Use 'w' for a 0.5s pause.
    pub post_dial_digits: Option<TwilioWarmConferenceTransferConfigOutputPostDialDigits>,
}

impl TwilioWarmConferenceTransferConfigOutput {
    pub fn builder() -> TwilioWarmConferenceTransferConfigOutputBuilder {
        <TwilioWarmConferenceTransferConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioWarmConferenceTransferConfigOutputBuilder {
    transfer_destination: Option<TwilioWarmConferenceTransferConfigOutputTransferDestination>,
    post_dial_digits: Option<TwilioWarmConferenceTransferConfigOutputPostDialDigits>,
}

impl TwilioWarmConferenceTransferConfigOutputBuilder {
    pub fn transfer_destination(
        mut self,
        value: TwilioWarmConferenceTransferConfigOutputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn post_dial_digits(
        mut self,
        value: TwilioWarmConferenceTransferConfigOutputPostDialDigits,
    ) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioWarmConferenceTransferConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](TwilioWarmConferenceTransferConfigOutputBuilder::transfer_destination)
    pub fn build(self) -> Result<TwilioWarmConferenceTransferConfigOutput, BuildError> {
        Ok(TwilioWarmConferenceTransferConfigOutput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            post_dial_digits: self.post_dial_digits,
        })
    }
}
