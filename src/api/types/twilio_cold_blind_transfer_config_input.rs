pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwilioColdBlindTransferConfigInput {
    pub transfer_destination: TwilioColdBlindTransferConfigInputTransferDestination,
    /// DTMF digits to send after the call connects (e.g. 'ww1234' for an extension). Static or a dynamic variable reference. Use 'w' for a 0.5s pause.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<TwilioColdBlindTransferConfigInputPostDialDigits>,
}

impl TwilioColdBlindTransferConfigInput {
    pub fn builder() -> TwilioColdBlindTransferConfigInputBuilder {
        <TwilioColdBlindTransferConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioColdBlindTransferConfigInputBuilder {
    transfer_destination: Option<TwilioColdBlindTransferConfigInputTransferDestination>,
    post_dial_digits: Option<TwilioColdBlindTransferConfigInputPostDialDigits>,
}

impl TwilioColdBlindTransferConfigInputBuilder {
    pub fn transfer_destination(
        mut self,
        value: TwilioColdBlindTransferConfigInputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn post_dial_digits(
        mut self,
        value: TwilioColdBlindTransferConfigInputPostDialDigits,
    ) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioColdBlindTransferConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](TwilioColdBlindTransferConfigInputBuilder::transfer_destination)
    pub fn build(self) -> Result<TwilioColdBlindTransferConfigInput, BuildError> {
        Ok(TwilioColdBlindTransferConfigInput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            post_dial_digits: self.post_dial_digits,
        })
    }
}
