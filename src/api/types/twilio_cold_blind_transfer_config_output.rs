pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwilioColdBlindTransferConfigOutput {
    pub transfer_destination: TwilioColdBlindTransferConfigOutputTransferDestination,
    /// DTMF digits to send after the call connects (e.g. 'ww1234' for an extension). Static or a dynamic variable reference. Use 'w' for a 0.5s pause.
    pub post_dial_digits: Option<TwilioColdBlindTransferConfigOutputPostDialDigits>,
}

impl TwilioColdBlindTransferConfigOutput {
    pub fn builder() -> TwilioColdBlindTransferConfigOutputBuilder {
        <TwilioColdBlindTransferConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioColdBlindTransferConfigOutputBuilder {
    transfer_destination: Option<TwilioColdBlindTransferConfigOutputTransferDestination>,
    post_dial_digits: Option<TwilioColdBlindTransferConfigOutputPostDialDigits>,
}

impl TwilioColdBlindTransferConfigOutputBuilder {
    pub fn transfer_destination(
        mut self,
        value: TwilioColdBlindTransferConfigOutputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn post_dial_digits(
        mut self,
        value: TwilioColdBlindTransferConfigOutputPostDialDigits,
    ) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioColdBlindTransferConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](TwilioColdBlindTransferConfigOutputBuilder::transfer_destination)
    pub fn build(self) -> Result<TwilioColdBlindTransferConfigOutput, BuildError> {
        Ok(TwilioColdBlindTransferConfigOutput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            post_dial_digits: self.post_dial_digits,
        })
    }
}
